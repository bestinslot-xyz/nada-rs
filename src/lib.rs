mod encoder;
use encoder::Encoder;

mod decoder;
pub use decoder::DecodeError;
use decoder::Decoder;

pub fn encode(input: impl IntoIterator<Item = u8>) -> Vec<u8> {
    let mut encoder = Encoder::new();

    for byte in input {
        encoder.feed(byte);
    }

    encoder.output()
}

pub fn decode(input: impl IntoIterator<Item = u8>) -> Result<Vec<u8>, DecodeError> {
    let mut decoder = Decoder::new();
    let mut iter = input.into_iter();

    while let Some(byte) = iter.next() {
        decoder.feed(byte)?;
    }

    decoder.output()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_decode() {
        let data = vec![0, 0, 0xFF, 0, 0, 0, 0xFF, 0];
        let encoded = encode(data.clone());
        let decoded = decode(encoded).unwrap();
        assert_eq!(data, decoded);
    }

    #[test]
    fn test_encode_decode_empty() {
        let data: Vec<u8> = vec![];
        let encoded = encode(data.clone());
        let decoded = decode(encoded);
        assert_eq!(Ok(data), decoded);
    }

    #[test]
    fn test_encode_decode_no_zeros() {
        let data = vec![1, 2, 3, 4, 5];
        let encoded = encode(data.clone());
        let decoded = decode(encoded);
        assert_eq!(Ok(data), decoded);
    }

    #[test]
    fn test_encode_decode_no_zeros_or_ff() {
        let data = vec![1, 2, 3, 4, 5];
        let encoded = encode(data.clone());
        let decoded = decode(encoded);
        assert_eq!(Ok(data), decoded);
    }

    #[test]
    fn test_encode_decode_only_zeros() {
        let data = vec![0, 0, 0, 0];
        let encoded = encode(data.clone());
        assert_eq!(encoded, vec![0xFF, 4]);
        let decoded = decode(encoded);
        assert_eq!(Ok(data), decoded);
    }

    #[test]
    fn test_encode_decode_only_ff() {
        let data = vec![0xFF, 0xFF, 0xFF];
        let encoded = encode(data.clone());
        assert_eq!(encoded, vec![0xFF, 2, 0xFF, 1]);
        let decoded = decode(encoded);
        assert_eq!(Ok(data), decoded);
    }

    #[test]
    fn test_encode_decode_mixed() {
        let data = vec![0, 0xFF, 1, 2, 0, 0, 0xFF, 3];
        let encoded = encode(data.clone());
        let decoded = decode(encoded);
        assert_eq!(Ok(data), decoded);
    }

    #[test]
    fn test_encode_decode_large_run() {
        let data = vec![0; 300];
        let encoded = encode(data.clone());
        assert_eq!(encoded, vec![0xFF, 255, 0xFF, 45]);
        let decoded = decode(encoded);
        assert_eq!(Ok(data), decoded);
    }

    #[test]
    fn test_encode_decode_large_run_with_ff() {
        let mut data = vec![0; 300];
        data[100] = 0xFF;
        let encoded = encode(data.clone());
        assert_eq!(encoded, vec![0xFF, 100, 0xFF, 1, 0xFF, 199]);
        let decoded = decode(encoded);
        assert_eq!(Ok(data), decoded);
    }

    #[test]
    fn test_error_unexpected_eof() {
        let data = vec![0xFF];
        let result = decode(data);
        assert_eq!(result, Err(DecodeError::UnexpectedEOF));
    }

    #[test]
    fn test_error_unexpected_eof_with_zeros() {
        let data = vec![0xFF, 25, 0xFF];
        let result = decode(data);
        assert_eq!(result, Err(DecodeError::UnexpectedEOF));
    }

    #[test]
    fn test_reserved_sequence() {
        let encoded = vec![0xFF, 0x01, 0xFF, 0x05, 2, 0xFF, 0, 0x03];
        assert_eq!(decode(encoded), Err(DecodeError::ReservedSequence));
    }
}
