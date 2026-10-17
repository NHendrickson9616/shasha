//! Minimal causal transformer and controller-routed action heads.

use candle_core::{Result, Tensor};
use candle_nn::{Embedding, LayerNorm, Linear, Module, VarBuilder, embedding, layer_norm, linear};

use crate::controller::{Need, SymbolId};

pub const VOCAB_SIZE: usize = 256;
const WIDTH: usize = 16;
const HEADS: usize = 2;
pub const MAX_LEN: usize = 128;

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
    no_parameter_head: Linear,
    parameter_head: Linear,
    two_parameter_head: Linear,
    symbol_key: Embedding,
    symbol_query: Linear,
    name_start_head: Linear,
    name_continue_head: Linear,
}

impl TinyTransformer {
    pub fn new(vb: VarBuilder<'_>) -> Result<Self> {
        Ok(Self {
            token: embedding(VOCAB_SIZE, WIDTH, vb.pp("token"))?,
            position: embedding(MAX_LEN, WIDTH, vb.pp("position"))?,
            // Previous actions use disjoint slots for each head's local action IDs.
            action: embedding(67, WIDTH, vb.pp("action"))?,
            need: embedding(7, WIDTH, vb.pp("need"))?,
            q: linear(WIDTH, WIDTH, vb.pp("q"))?,
            k: linear(WIDTH, WIDTH, vb.pp("k"))?,
            v: linear(WIDTH, WIDTH, vb.pp("v"))?,
            attention_out: linear(WIDTH, WIDTH, vb.pp("attention_out"))?,
            attention_norm: layer_norm(WIDTH, 1e-5, vb.pp("attention_norm"))?,
            mlp_norm: layer_norm(WIDTH, 1e-5, vb.pp("mlp_norm"))?,
            mlp_in: linear(WIDTH, 32, vb.pp("mlp_in"))?,
            mlp_out: linear(32, WIDTH, vb.pp("mlp_out"))?,
            root_head: linear(WIDTH, 3, vb.pp("root_head"))?,
            no_parameter_head: linear(WIDTH, 4, vb.pp("no_parameter_head"))?,
            parameter_head: linear(WIDTH, 2, vb.pp("parameter_head"))?,
            two_parameter_head: linear(WIDTH, 3, vb.pp("two_parameter_head"))?,
            symbol_key: embedding(2, WIDTH, vb.pp("symbol_key"))?,
            symbol_query: linear(WIDTH, WIDTH, vb.pp("symbol_query"))?,
            name_start_head: linear(WIDTH, 26, vb.pp("name_start_head"))?,
            name_continue_head: linear(WIDTH, 27, vb.pp("name_continue_head"))?,
        })
    }

    /// Scores the next controller decision as `[1, number_of_candidates]`.
    pub fn score(
        &self,
        prompt: &[u32],
        history: &[(Need, usize)],
        current: Need,
        symbols: &[SymbolId],
    ) -> Result<Tensor> {
        if current == Need::SymbolPointer {
            if symbols.is_empty()
                || symbols.len() > 2
                || symbols.iter().any(|symbol| !(1..=2).contains(&symbol.0))
                || (symbols.len() == 2 && symbols[0] == symbols[1])
            {
                candle_core::bail!(
                    "pointer head requires one or two distinct live symbol IDs (1 or 2)"
                )
            }
        } else if !symbols.is_empty() {
            candle_core::bail!("fixed heads require an empty symbol list")
        }
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
                Need::Root if choice < 3 => choice,
                Need::NoParameterExpression if choice < 4 => 3 + choice,
                Need::ParameterExpression if choice < 2 => 7 + choice,
                Need::TwoParameterExpression if choice < 3 => 9 + choice,
                Need::SymbolPointer if choice < 2 => 12 + choice,
                Need::NameStart if choice < 26 => 14 + choice,
                Need::NameContinue if choice < 27 => 40 + choice,
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
            Need::NoParameterExpression => self.no_parameter_head.forward(&hidden),
            Need::ParameterExpression => self.parameter_head.forward(&hidden),
            Need::TwoParameterExpression => self.two_parameter_head.forward(&hidden),
            Need::SymbolPointer => {
                let indices: Vec<u32> = symbols.iter().map(|symbol| symbol.0 - 1).collect();
                let keys = self
                    .symbol_key
                    .forward(&Tensor::new(indices.as_slice(), device)?)?;
                self.symbol_query
                    .forward(&hidden)?
                    .matmul(&keys.t()?.contiguous()?)
            }
            Need::NameStart => self.name_start_head.forward(&hidden),
            Need::NameContinue => self.name_continue_head.forward(&hidden),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use candle_core::{DType, Device};
    use candle_nn::VarMap;

    fn model() -> Result<TinyTransformer> {
        let vars = VarMap::new();
        TinyTransformer::new(VarBuilder::from_varmap(&vars, DType::F32, &Device::Cpu))
    }

    #[test]
    fn only_the_selected_head_scores_its_local_choices() -> Result<()> {
        let model = model()?;
        assert_eq!(model.action.embeddings().dims(), &[67, WIDTH]);
        for (need, count) in [
            (Need::Root, 3),
            (Need::NoParameterExpression, 4),
            (Need::ParameterExpression, 2),
            (Need::TwoParameterExpression, 3),
            (Need::NameStart, 26),
            (Need::NameContinue, 27),
        ] {
            assert_eq!(model.score(&[1], &[], need, &[])?.dims(), &[1, count]);
            assert!(model.score(&[1], &[], need, &[SymbolId(1)]).is_err());
        }
        Ok(())
    }

    #[test]
    fn pointer_scores_live_symbols_in_provided_order() -> Result<()> {
        let model = model()?;
        let first = model.score(&[1], &[], Need::SymbolPointer, &[SymbolId(1)])?;
        assert_eq!(first.dims(), &[1, 1]);
        let second = model.score(&[1], &[], Need::SymbolPointer, &[SymbolId(2)])?;
        assert_eq!(second.dims(), &[1, 1]);
        let forward = model.score(&[1], &[], Need::SymbolPointer, &[SymbolId(1), SymbolId(2)])?;
        let reverse = model.score(&[1], &[], Need::SymbolPointer, &[SymbolId(2), SymbolId(1)])?;
        assert_eq!(forward.dims(), &[1, 2]);
        assert_eq!(reverse.dims(), &[1, 2]);
        let forward = forward.to_vec2::<f32>()?[0].clone();
        let reverse = reverse.to_vec2::<f32>()?[0].clone();
        assert!((forward[0] - reverse[1]).abs() < 1e-5);
        assert!((forward[1] - reverse[0]).abs() < 1e-5);
        assert!((first.to_vec2::<f32>()?[0][0] - forward[0]).abs() < 1e-5);
        assert!((second.to_vec2::<f32>()?[0][0] - forward[1]).abs() < 1e-5);
        for invalid in [
            vec![],
            vec![SymbolId(0)],
            vec![SymbolId(3)],
            vec![SymbolId(1), SymbolId(1)],
            vec![SymbolId(1), SymbolId(2), SymbolId(1)],
        ] {
            assert!(
                model
                    .score(&[1], &[], Need::SymbolPointer, &invalid)
                    .is_err()
            );
        }
        Ok(())
    }

    #[test]
    fn history_slots_accept_only_local_choices() -> Result<()> {
        let model = model()?;
        for choice in 0..3 {
            assert_eq!(
                model
                    .score(
                        &[1],
                        &[(Need::TwoParameterExpression, choice)],
                        Need::Root,
                        &[]
                    )?
                    .dims(),
                &[1, 3]
            );
        }
        for (need, last_choice) in [
            (Need::Root, 2),
            (Need::NoParameterExpression, 3),
            (Need::ParameterExpression, 1),
            (Need::TwoParameterExpression, 2),
            (Need::SymbolPointer, 1),
            (Need::NameStart, 25),
            (Need::NameContinue, 26),
        ] {
            assert_eq!(
                model
                    .score(&[1], &[(need, last_choice)], Need::Root, &[])?
                    .dims(),
                &[1, 3]
            );
            assert!(
                model
                    .score(&[1], &[(need, last_choice + 1)], Need::Root, &[])
                    .is_err()
            );
        }
        Ok(())
    }
}
