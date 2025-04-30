pub(crate) struct Encoder {
    zero_run: u8,
    ff_run: u8,
    out: Vec<u8>,
}

impl Encoder {
    pub fn new() -> Self {
        Encoder {
            zero_run: 0,
            ff_run: 0,
            out: Vec::new(),
        }
    }

    fn flush_zeroes(&mut self) {
        match self.zero_run {
            0 => {}
            1 => {
                self.out.push(0);
                self.zero_run = 0;
            }
            2 => {
                self.out.push(0);
                self.out.push(0);
                self.zero_run = 0;
            }
            3..=255 => {
                self.out.push(0xFF);
                self.out.push(self.zero_run);
                self.zero_run = 0;
            }
        }
    }

    fn flush_ff(&mut self) {
        match self.ff_run {
            0 => {}
            1 => {
                self.out.push(0xFF);
                self.out.push(1);
                self.ff_run = 0;
            }
            2 => {
                self.out.push(0xFF);
                self.out.push(2);
                self.ff_run = 0;
            }
            _ => {
                // Should be impossible to reach here
                panic!("ff_run should never be greater than 2");
            }
        }
    }

    fn flush(&mut self) {
        self.flush_zeroes();
        self.flush_ff();
    }

    fn feed_zero(&mut self) {
        self.flush_ff();
        self.zero_run += 1;
        if self.zero_run == 255 {
            self.flush_zeroes();
        }
    }

    fn feed_ff(&mut self) {
        self.flush_zeroes();
        self.ff_run += 1;
        if self.ff_run == 2 {
            self.flush_ff();
        }
    }

    pub fn feed(&mut self, byte: u8) {
        if byte == 0 {
            self.feed_zero();
        } else if byte == 0xFF {
            self.feed_ff();
        } else {
            self.flush();
            self.out.push(byte);
        }
    }

    pub fn output(&mut self) -> Vec<u8> {
        self.flush();
        self.out.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        let mut encoder = Encoder::new();
        encoder.feed(0);
        encoder.feed(0);
        encoder.feed(0xFF);
        encoder.feed(0);
        encoder.feed(0);
        encoder.feed(0);
        encoder.feed(0xFF);
        encoder.feed(0);

        let encoded = encoder.output();
        assert_eq!(encoded, vec![0, 0, 0xFF, 1, 0xFF, 3, 0xFF, 1, 00]);
    }

    #[test]
    fn test_encode_no_zeros() {
        let mut encoder = Encoder::new();
        encoder.feed(1);
        encoder.feed(2);
        encoder.feed(3);
        encoder.feed(4);
        encoder.feed(5);

        let encoded = encoder.output();
        assert_eq!(encoded, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_encode_all_ff() {
        let mut encoder = Encoder::new();
        encoder.feed(0xFF);
        encoder.feed(0xFF);
        encoder.feed(0xFF);
        encoder.feed(0xFF);
        encoder.feed(0xFF);

        let encoded = encoder.output();
        assert_eq!(encoded, vec![0xFF, 2, 0xFF, 2, 0xFF, 1]);
    }

    #[test]
    fn test_encode_only_zeros() {
        let mut encoder = Encoder::new();
        encoder.feed(0);
        encoder.feed(0);
        encoder.feed(0);
        encoder.feed(0);

        let encoded = encoder.output();
        assert_eq!(encoded, vec![0xFF, 4]);
    }

    #[test]
    fn test_encode_only_ff() {
        let mut encoder = Encoder::new();
        encoder.feed(0xFF);
        encoder.feed(0xFF);
        encoder.feed(0xFF);

        let encoded = encoder.output();
        assert_eq!(encoded, vec![0xFF, 2, 0xFF, 1]);
    }

    #[test]
    fn test_encode_empty() {
        let mut encoder = Encoder::new();
        let encoded = encoder.output();
        assert_eq!(encoded, vec![]);
    }

    #[test]
    fn test_encode_mixed() {
        let mut encoder = Encoder::new();
        encoder.feed(0);
        encoder.feed(0xFF);
        encoder.feed(1);
        encoder.feed(2);
        encoder.feed(0);
        encoder.feed(0);
        encoder.feed(0xFF);
        encoder.feed(3);

        let encoded = encoder.output();
        assert_eq!(encoded, vec![0, 0xFF, 1, 1, 2, 0, 0, 0xFF, 1, 3]);
    }

    #[test]
    fn test_encode_mixed_zeros() {
        let mut encoder = Encoder::new();
        encoder.feed(0);
        encoder.feed(0xFF);
        encoder.feed(1);
        encoder.feed(0);
        encoder.feed(0);
        encoder.feed(0);
        encoder.feed(0);
        encoder.feed(0);
        encoder.feed(0);
        encoder.feed(2);
        encoder.feed(0);
        encoder.feed(0);
        encoder.feed(0);
        encoder.feed(0);
        encoder.feed(0xFF);
        encoder.feed(3);

        let encoded = encoder.output();
        assert_eq!(
            encoded,
            vec![0, 0xFF, 1, 1, 0xFF, 6, 2, 0xFF, 4, 0xFF, 1, 3]
        );
    }

    #[test]
    fn test_encode_mixed_ff() {
        let mut encoder = Encoder::new();
        encoder.feed(0);
        encoder.feed(0xFF);
        encoder.feed(1);
        encoder.feed(0xFF);
        encoder.feed(0xFF);
        encoder.feed(0xFF);
        encoder.feed(2);
        encoder.feed(0xFF);
        encoder.feed(3);

        let encoded = encoder.output();
        assert_eq!(
            encoded,
            vec![0, 0xFF, 1, 1, 0xFF, 2, 0xFF, 1, 2, 0xFF, 1, 3]
        );
    }
}
