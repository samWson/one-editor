use std::fmt;

const DEFAULT_BUFFER_CAPACITY: usize = 10;

struct GapBuffer {
    buffer: Vec<char>,
}

impl GapBuffer {
    fn new() -> GapBuffer {
        GapBuffer {
            buffer: Vec::with_capacity(DEFAULT_BUFFER_CAPACITY),
        }
    }

    fn capacity(&self) -> usize {
        self.buffer.capacity()
    }

    fn len(&self) -> usize {
        self.buffer.len()
    }
}

impl fmt::Display for GapBuffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

#[cfg(test)]
mod tests {
    use GapBuffer;
    use DEFAULT_BUFFER_CAPACITY;

    #[test]
    fn initialized_empty() {
        let buffer = GapBuffer::new();

        assert_eq!(buffer.capacity(), DEFAULT_BUFFER_CAPACITY);
        assert_eq!(buffer.len(), 0);
        assert_eq!(buffer.to_string(), "")
    }
}
