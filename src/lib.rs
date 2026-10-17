//! Small, independent starter for Shasha's structured generation path.

pub mod controller;
pub mod model;
pub mod tokenizer;
pub mod validation;

use candle_core::Result;
use controller::{Controller, Need};
use model::TinyTransformer;

/// A selected head and its head-local action ID.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraceStep {
    pub need: Need,
    pub choice: usize,
    pub description: String,
}

/// Output of a prompt, including the actual bytes and decisions used at inference.
#[derive(Debug)]
pub struct Generation {
    pub code: String,
    pub token_ids: Vec<u32>,
    pub steps: Vec<TraceStep>,
}

/// Convert UTF-8 text to byte IDs and run the same inference path as `generate`.
/// Without training, changing the prompt does not produce meaningful instructions.
pub fn generate_text(model: &TinyTransformer, prompt: &str) -> Result<Generation> {
    let token_ids =
        tokenizer::encode(prompt).map_err(|error| candle_core::Error::Msg(error.to_owned()))?;
    let (code, steps) = decode(model, &token_ids)?;
    Ok(Generation {
        code,
        token_ids,
        steps,
    })
}

/// Decode structured actions, pointers, and names from already-tokenized input.
pub fn generate(model: &TinyTransformer, prompt_ids: &[u32]) -> Result<String> {
    decode(model, prompt_ids).map(|(code, _)| code)
}

fn decode(model: &TinyTransformer, prompt_ids: &[u32]) -> Result<(String, Vec<TraceStep>)> {
    let mut controller = Controller::new();
    let mut history = Vec::new();
    let mut steps = Vec::new();
    while let Some(step) = controller.step() {
        let logits = model.score(prompt_ids, &history, step.need, &step.symbols)?;
        if logits.dims() != [1, step.candidate_count()] {
            candle_core::bail!("selected head returned an unexpected number of choices")
        }
        let choice = logits.argmax(1)?.to_vec1::<u32>()?[0] as usize;
        let description = step.describe(choice);
        controller
            .apply(choice)
            .map_err(|error| candle_core::Error::Msg(error.to_owned()))?;
        history.push((step.need, choice));
        steps.push(TraceStep {
            need: step.need,
            choice,
            description,
        });
    }
    let code = controller
        .render()
        .map_err(|error| candle_core::Error::Msg(error.to_owned()))?;
    validation::validate_rust(&code).map_err(candle_core::Error::Msg)?;
    Ok((code, steps))
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
        assert!(rendered.starts_with("fn n_"));
        assert!(rendered.contains(") -> i32 { "));
        assert!(rendered.ends_with(" }\n"));
        Ok(())
    }

    #[test]
    fn text_prompt_reaches_both_action_heads() -> Result<()> {
        let vars = VarMap::new();
        let model = TinyTransformer::new(VarBuilder::from_varmap(&vars, DType::F32, &Device::Cpu))?;
        let result = generate_text(&model, "Increment 1 🦀")?;
        assert_eq!(
            result.token_ids,
            "Increment 1 🦀"
                .as_bytes()
                .iter()
                .map(|&b| u32::from(b))
                .collect::<Vec<_>>()
        );
        assert!(result.steps.len() >= 4);
        assert_eq!(result.steps[0].need, Need::Root);
        assert!(result.steps.iter().any(|step| step.need == Need::NameStart));
        assert!(result.code.starts_with("fn n_"));
        Ok(())
    }

    #[test]
    fn longest_supported_prompt_leaves_room_for_names_and_pointers() -> Result<()> {
        let vars = VarMap::new();
        let model = TinyTransformer::new(VarBuilder::from_varmap(&vars, DType::F32, &Device::Cpu))?;
        let result = generate_text(&model, &"a".repeat(tokenizer::MAX_PROMPT_BYTES))?;
        assert_eq!(result.token_ids.len(), tokenizer::MAX_PROMPT_BYTES);
        assert!(result.code.starts_with("fn n_"));
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
