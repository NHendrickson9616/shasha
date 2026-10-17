//! Lossless UTF-8 byte tokenization for the tiny demo (not a trained tokenizer).

use crate::model::MAX_LEN;

/// Reserve positions for structure, two pointers, and three bounded names.
pub const MAX_PROMPT_BYTES: usize = MAX_LEN - 14;

pub fn encode(prompt: &str) -> Result<Vec<u32>, &'static str> {
    if prompt.trim().is_empty() {
        return Err("prompt must contain non-whitespace text");
    }
    if prompt.len() > MAX_PROMPT_BYTES {
        return Err("prompt is too long for the model's sequence length");
    }
    Ok(prompt
        .as_bytes()
        .iter()
        .map(|&byte| u32::from(byte))
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn utf8_round_trips_as_bytes() {
        let text = "Rust 🦀";
        let tokens = encode(text).unwrap();
        assert_eq!(
            tokens,
            text.as_bytes()
                .iter()
                .map(|&b| u32::from(b))
                .collect::<Vec<_>>()
        );
        assert_eq!(
            String::from_utf8(tokens.iter().map(|&id| id as u8).collect()).unwrap(),
            text
        );
    }

    #[test]
    fn validates_byte_length_and_empty_text() {
        assert!(encode("  ").is_err());
        assert_eq!(
            encode(&"a".repeat(MAX_PROMPT_BYTES)).unwrap().len(),
            MAX_PROMPT_BYTES
        );
        assert!(encode(&"a".repeat(MAX_PROMPT_BYTES + 1)).is_err());
        assert!(encode(&"🦀".repeat(MAX_PROMPT_BYTES / 4 + 1)).is_err());
    }
}
