use candle_core::{DType, Device};
use candle_nn::{VarBuilder, VarMap};
use mopappa_hackathon::{generate, model::TinyTransformer};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let vars = VarMap::new();
    let model = TinyTransformer::new(VarBuilder::from_varmap(&vars, DType::F32, &Device::Cpu))?;
    // Replace these stand-in token IDs with IDs from a real prompt tokenizer.
    let rust = generate(&model, &[2, 5])?;
    println!("Untrained sample (not a meaningful prediction):\n{rust}");
    Ok(())
}
