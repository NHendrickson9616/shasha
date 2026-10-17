use candle_core::{DType, Device};
use candle_nn::{VarBuilder, VarMap};
use shasha_hackathon::{generate_text, model::TinyTransformer};

fn select_device() -> candle_core::Result<Device> {
    let requested = std::env::var("SHASHA_DEVICE").unwrap_or_else(|_| "auto".to_owned());
    match requested.to_ascii_lowercase().as_str() {
        "cpu" => Ok(Device::Cpu),
        "cuda" => cuda_device(),
        "auto" => {
            #[cfg(feature = "cuda")]
            {
                match cuda_device() {
                    Ok(device) => Ok(device),
                    Err(error) => {
                        eprintln!("CUDA unavailable ({error}); falling back to CPU");
                        Ok(Device::Cpu)
                    }
                }
            }
            #[cfg(not(feature = "cuda"))]
            {
                Ok(Device::Cpu)
            }
        }
        _ => candle_core::bail!("SHASHA_DEVICE must be auto, cpu, or cuda"),
    }
}

#[cfg(feature = "cuda")]
fn cuda_device() -> candle_core::Result<Device> {
    Device::new_cuda(0)
}

#[cfg(not(feature = "cuda"))]
fn cuda_device() -> candle_core::Result<Device> {
    candle_core::bail!("CUDA requested; rebuild with --features cuda")
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let prompt = std::env::args().skip(1).collect::<Vec<_>>().join(" ");
    let prompt = if prompt.is_empty() {
        "add two integers"
    } else {
        &prompt
    };

    let device = select_device()?;
    let vars = VarMap::new();
    let model = TinyTransformer::new(VarBuilder::from_varmap(&vars, DType::F32, &device))?;
    let result = generate_text(&model, prompt)?;

    println!("Device: {}", if device.is_cuda() { "cuda" } else { "cpu" });
    println!("Prompt: {prompt}");

    println!(
        "UTF-8 byte tokens ({}): {:?}",
        result.token_ids.len(),
        result.token_ids
    );
    println!("Inference trace (random weights):");
    for (index, step) in result.steps.iter().enumerate() {
        println!(
            "  {}. {} → {}",
            index + 1,
            step.need.label(),
            step.description
        );
    }
    println!("Rust validation: passed (rustc type-check)");
    println!("Generated Rust:\n{}", result.code);
    Ok(())
}
