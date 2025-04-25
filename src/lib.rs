static MARKER: u8 = 0xFF;

pub fn encode(input: impl IntoIterator<Item = u8>) -> Vec<u8> {
    let mut out = Vec::new();
    let mut zero_run = 0;

    for byte in input {
        if byte == 0 {
            zero_run += 1;
            // Max run length
            if zero_run == 255 {
                out.push(MARKER);
                out.push(255);
                zero_run = 0;
            }
        } else {
            if zero_run > 0 {
                if zero_run == 1 {
                    out.push(0);
                } else {
                    out.push(MARKER);
                    out.push(zero_run);
                }
                zero_run = 0;
            }
            if byte == MARKER {
                // Escape literal 0xFF to avoid confusion with marker
                out.push(MARKER);
                out.push(0);
            } else {
                out.push(byte);
            }
        }
    }

    if zero_run > 0 {
        out.push(0xFF);
        out.push(zero_run);
    }

    out
}

pub fn decode(input: impl IntoIterator<Item = u8>) -> Result<Vec<u8>, DecodeError> {
    let mut out = Vec::new();
    let mut iter = input.into_iter();

    while let Some(byte) = iter.next() {
        if byte == 0xFF {
            if let Some(n) = iter.next() {
                if n == 0 {
                    out.push(0xFF);
                } else {
                    out.extend(std::iter::repeat(0).take(n as usize));
                }
            } else {
                return Err(DecodeError::UnexpectedEOF);
            }
        } else {
            out.push(byte);
        }
    }

    Ok(out)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DecodeError {
    UnexpectedEOF, // Reached the end of input unexpectedly
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
        assert_eq!(encoded, vec![0xFF, 0, 0xFF, 0, 0xFF, 0]);
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
        assert_eq!(encoded, vec![0xFF, 100, 0xFF, 0, 0xFF, 199]);
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
}
