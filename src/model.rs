//! Minimal causal transformer and controller-routed action heads.

use candle_core::{DType, Result, Tensor};
use candle_nn::{Embedding, LayerNorm, Linear, Module, VarBuilder, embedding, layer_norm, linear};

use crate::controller::Need;

pub const VOCAB_SIZE: usize = 64;
const WIDTH: usize = 16;
const HEADS: usize = 2;
const MAX_LEN: usize = 32;

pub struct TinyTransformer {
    token: Embedding,
    position: Embedding,
    action: Embedding,
    need: Embedding,
    q: Linear,
    k: Linear,
    v: Linear,
    attention_out: Linear,
    attention_norm: LayerNorm,
    mlp_norm: LayerNorm,
    mlp_in: Linear,
    mlp_out: Linear,
    root_head: Linear,
    expression_head: Linear,
}

impl TinyTransformer {
    pub fn new(vb: VarBuilder<'_>) -> Result<Self> {
        Ok(Self {
            token: embedding(VOCAB_SIZE, WIDTH, vb.pp("token"))?,
            position: embedding(MAX_LEN, WIDTH, vb.pp("position"))?,
            // IDs 0..2 denote root actions; 2..5 denote expression actions.
            action: embedding(5, WIDTH, vb.pp("action"))?,
            need: embedding(2, WIDTH, vb.pp("need"))?,
            q: linear(WIDTH, WIDTH, vb.pp("q"))?,
            k: linear(WIDTH, WIDTH, vb.pp("k"))?,
            v: linear(WIDTH, WIDTH, vb.pp("v"))?,
            attention_out: linear(WIDTH, WIDTH, vb.pp("attention_out"))?,
            attention_norm: layer_norm(WIDTH, 1e-5, vb.pp("attention_norm"))?,
            mlp_norm: layer_norm(WIDTH, 1e-5, vb.pp("mlp_norm"))?,
            mlp_in: linear(WIDTH, 32, vb.pp("mlp_in"))?,
            mlp_out: linear(32, WIDTH, vb.pp("mlp_out"))?,
            root_head: linear(WIDTH, 2, vb.pp("root_head"))?,
            expression_head: linear(WIDTH, 3, vb.pp("expression_head"))?,
        })
    }

    /// Scores the next controller decision as `[1, number_of_candidates]`.
    pub fn score(
        &self,
        prompt: &[u32],
        history: &[(Need, usize)],
        current: Need,
    ) -> Result<Tensor> {
        let len = prompt.len() + history.len() + 1;
        if prompt.is_empty() || len > MAX_LEN || prompt.iter().any(|&id| id as usize >= VOCAB_SIZE)
        {
            candle_core::bail!(
                "prompt must contain in-vocabulary IDs and fit within {MAX_LEN} positions"
            )
        }
        let device = self.token.embeddings().device();
        let mut rows = Vec::with_capacity(len);
        for &id in prompt {
            rows.push(self.token.forward(&Tensor::new(&[id], device)?)?);
        }
        for &(need, choice) in history {
            let action_id = match need {
                Need::Root if choice < 2 => choice,
                Need::Expression if choice < 3 => 2 + choice,
                _ => candle_core::bail!("invalid action history"),
            };
            let action = self
                .action
                .forward(&Tensor::new(&[action_id as u32], device)?)?;
            let need = self.need.forward(&Tensor::new(&[need.id()], device)?)?;
            rows.push((&action + &need)?);
        }
        rows.push(self.need.forward(&Tensor::new(&[current.id()], device)?)?);
        let sequence = Tensor::cat(&rows, 0)?;
        let positions = Tensor::new((0..len as u32).collect::<Vec<_>>().as_slice(), device)?;
        let input = (sequence + self.position.forward(&positions)?)?.unsqueeze(0)?;

        let normalized = self.attention_norm.forward(&input)?;
        let project = |layer: &Linear| {
            layer
                .forward(&normalized)?
                .reshape((1, len, HEADS, WIDTH / HEADS))?
                .transpose(1, 2)?
                .contiguous()
        };
        let q = project(&self.q)?;
        let k = project(&self.k)?;
        let v = project(&self.v)?;
        let scores =
            (q.matmul(&k.transpose(2, 3)?.contiguous()?)? / ((WIDTH / HEADS) as f64).sqrt())?;
        let mask: Vec<f32> = (0..len)
            .flat_map(|row| {
                (0..len).map(move |col| if col > row { f32::NEG_INFINITY } else { 0.0 })
            })
            .collect();
        let mask = Tensor::from_vec(mask, (1, 1, len, len), device)?;
        let weights = candle_nn::ops::softmax_last_dim(&scores.broadcast_add(&mask)?)?;
        let context = weights
            .matmul(&v)?
            .transpose(1, 2)?
            .contiguous()?
            .reshape((1, len, WIDTH))?;
        let hidden = (&input + self.attention_out.forward(&context)?)?;
        let feed = self.mlp_out.forward(
            &self
                .mlp_in
                .forward(&self.mlp_norm.forward(&hidden)?)?
                .gelu()?,
        )?;
        let hidden = (hidden + feed)?.narrow(1, len - 1, 1)?.squeeze(1)?;
        match current {
            Need::Root => self.root_head.forward(&hidden),
            Need::Expression => self.expression_head.forward(&hidden),
        }
    }
}

/// Mask illegal candidate IDs without changing the head's stable vocabulary.
pub fn mask_logits(logits: &Tensor, legal: &Tensor) -> Result<Tensor> {
    if logits.dims() != legal.dims() || legal.dtype() != DType::U8 {
        candle_core::bail!("legal mask must be U8 and have the same shape as logits")
    }
    let values = legal.to_vec2::<u8>()?;
    if values
        .iter()
        .any(|row| !row.contains(&1) || row.iter().any(|&value| value > 1))
    {
        candle_core::bail!("each mask row must contain a legal choice and only binary values")
    }
    let fill = Tensor::full(f32::NEG_INFINITY, logits.shape(), logits.device())?;
    legal.eq(0u8)?.where_cond(&fill, logits)
}

#[cfg(test)]
mod tests {
    use super::*;
    use candle_core::Device;
    use candle_nn::VarMap;

    #[test]
    fn heads_and_mask_keep_stable_candidate_slots() -> Result<()> {
        let vars = VarMap::new();
        let model = TinyTransformer::new(VarBuilder::from_varmap(&vars, DType::F32, &Device::Cpu))?;
        assert_eq!(model.score(&[1], &[], Need::Root)?.dims(), &[1, 2]);
        let logits = model.score(&[1], &[(Need::Root, 0)], Need::Expression)?;
        assert_eq!(logits.dims(), &[1, 3]);
        let legal = Tensor::new(&[[1u8, 1, 0]], &Device::Cpu)?;
        let masked = mask_logits(&logits, &legal)?.to_vec2::<f32>()?;
        assert_eq!(masked[0][2], f32::NEG_INFINITY);
        assert!(mask_logits(&logits, &Tensor::new(&[[0u8, 0, 0]], &Device::Cpu)?).is_err());
        Ok(())
    }
}
