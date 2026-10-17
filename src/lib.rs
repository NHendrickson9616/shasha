//! Small, independent starter inspired by MoPaPPa's structured generation path.

pub mod controller;
pub mod model;

use candle_core::{Result, Tensor};
use controller::Controller;
use model::TinyTransformer;

/// Decode two structural decisions. The randomly initialized model is a wiring
/// demo, not a trained English-to-Rust generator. Prompt IDs are placeholders.
pub fn generate(model: &TinyTransformer, prompt_ids: &[u32]) -> Result<String> {
    let mut controller = Controller::new();
    let mut history = Vec::new();
    while let Some(step) = controller.step() {
        let logits = model.score(prompt_ids, &history, step.need)?;
        let legal = Tensor::new(step.legal.as_slice(), logits.device())?.unsqueeze(0)?;
        let masked = model::mask_logits(&logits, &legal)?;
        let choice = masked.argmax(1)?.to_vec1::<u32>()?[0] as usize;
        controller
            .apply(choice)
            .map_err(|error| candle_core::Error::Msg(error.to_owned()))?;
        history.push((step.need, choice));
    }
    controller
        .render()
        .map_err(|error| candle_core::Error::Msg(error.to_owned()))
}

#[cfg(test)]
mod tests {
    use super::*;

    use candle_core::{DType, Device};
    use candle_nn::{VarBuilder, VarMap};

    #[test]
    fn untrained_model_completes_a_valid_program() -> Result<()> {
        let vars = VarMap::new();
        let model = TinyTransformer::new(VarBuilder::from_varmap(&vars, DType::F32, &Device::Cpu))?;
        let rendered = generate(&model, &[2, 5])?;
        assert!(rendered.starts_with("fn generated() -> i32 { ") || rendered.starts_with("{ "));
        assert!(rendered.ends_with(" }") || rendered.ends_with(" }\n"));
        Ok(())
    }

    #[test]
    fn empty_prompt_is_rejected() -> Result<()> {
        let vars = VarMap::new();
        let model = TinyTransformer::new(VarBuilder::from_varmap(&vars, DType::F32, &Device::Cpu))?;
        assert!(generate(&model, &[]).is_err());
        Ok(())
    }
}
