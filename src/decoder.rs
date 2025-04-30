pub(crate) struct Decoder {
    out: Vec<u8>,
    waiting_follow_up: bool,
}

impl Decoder {
    pub fn new() -> Self {
        Self {
            out: Vec::new(),
            waiting_follow_up: false,
        }
    }

    pub fn feed(&mut self, input: u8) -> Result<(), DecodeError> {
        if self.waiting_follow_up {
            match input {
                0x00 => {
                    return Err(DecodeError::ReservedSequence);
                }
                0x01 => {
                    self.waiting_follow_up = false;
                    self.out.push(0xFF);
                }
                0x02 => {
                    self.waiting_follow_up = false;
                    self.out.push(0xFF);
                    self.out.push(0xFF);
                }
                _ => {
                    self.waiting_follow_up = false;
                    for _ in 0..input {
                        self.out.push(0x00);
                    }
                }
            }
        } else {
            match input {
                0xFF => {
                    self.waiting_follow_up = true;
                }
                _ => {
                    self.out.push(input);
                }
            }
        }

        Ok(())
    }

    pub fn output(self) -> Result<Vec<u8>, DecodeError> {
        if self.waiting_follow_up {
            return Err(DecodeError::UnexpectedEOF);
        }
        Ok(self.out)
    }

    pub fn len(&self) -> usize {
        self.out.len()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DecodeError {
    UnexpectedEOF,    // Reached the end of input unexpectedly
    ReservedSequence, // Encountered a reserved sequence, e.g., 0xFF followed by 0x00
    LimitExceeded,    // Output exceeds the specified limit
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decoder() {
        let mut decoder = Decoder::new();
        let input = vec![0xFF, 0x01, 0xFF, 0x02, 0x03];
        for byte in input {
            decoder.feed(byte).unwrap();
        }
        let output = decoder.output().unwrap();
        assert_eq!(output, vec![0xFF, 0xFF, 0xFF, 0x03]);
    }

    #[test]
    fn test_decoder_empty() {
        let mut decoder = Decoder::new();
        let input: Vec<u8> = vec![];
        for byte in input {
            decoder.feed(byte).unwrap();
        }
        assert_eq!(decoder.output().unwrap(), vec![]);
    }

    #[test]
    fn test_decoder_no_zeros() {
        let mut decoder = Decoder::new();
        let input = vec![1, 2, 3, 4, 5];
        for byte in input {
            decoder.feed(byte).unwrap();
        }
        assert_eq!(decoder.output().unwrap(), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_decoder_only_zeros() {
        let mut decoder = Decoder::new();
        let input = vec![0xFF, 0x04];
        for byte in input {
            decoder.feed(byte).unwrap();
        }
        assert_eq!(decoder.output().unwrap(), vec![0, 0, 0, 0]);
    }

    #[test]
    fn test_decoder_only_ff() {
        let mut decoder = Decoder::new();
        let input = vec![0xFF, 0x02, 0xFF, 0x01];
        for byte in input {
            decoder.feed(byte).unwrap();
        }
        assert_eq!(decoder.output().unwrap(), vec![0xFF, 0xFF, 0xFF]);
    }

    #[test]
    fn test_decoder_mixed() {
        let mut decoder = Decoder::new();
        let input = vec![0xFF, 0x01, 0xFF, 0x05, 2, 0xFF, 1, 0x03];
        for byte in input {
            decoder.feed(byte).unwrap();
        }
        assert_eq!(
            decoder.output().unwrap(),
            vec![0xFF, 0, 0, 0, 0, 0, 2, 0xFF, 0x03]
        );
    }

    #[test]
    fn test_decoder_unexpected_eof() {
        let mut decoder = Decoder::new();
        let input = vec![1, 2, 3, 4, 5, 0xFF];
        for byte in input {
            decoder.feed(byte).unwrap();
        }
        assert_eq!(decoder.output(), Err(DecodeError::UnexpectedEOF));
    }

    #[test]
    fn test_decoder_reserved_sequence() {
        let mut decoder = Decoder::new();
        decoder.feed(0xFF).unwrap();
        assert_eq!(decoder.feed(0x00), Err(DecodeError::ReservedSequence));
    }
}
