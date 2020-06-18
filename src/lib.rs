use std::fmt;
use std::ops::Range;

const DEFAULT_BUFFER_CAPACITY: usize = 10;

struct GapBuffer {
    buffer: Vec<u8>,
}

impl GapBuffer {
    fn new() -> GapBuffer {
        GapBuffer {
            buffer: Vec::with_capacity(DEFAULT_BUFFER_CAPACITY),
        }
    }

    fn from(content: String) -> GapBuffer {
        GapBuffer {
            buffer: content.into_bytes(),
        }
    }

    fn capacity(&self) -> usize {
        self.buffer.capacity()
    }

    fn len(&self) -> usize {
        self.buffer.len()
    }

    fn insert(&mut self, index: usize, byte: u8) {
        self.buffer.insert(index, byte)
    }

    fn remove(&mut self, index: usize) -> u8 {
        self.buffer.remove(index)
    }

    fn remove_range(&mut self, range: Range<usize>) -> Vec<u8> {
        self.buffer.drain(range).collect()
    }
}

impl fmt::Display for GapBuffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", std::str::from_utf8(&self.buffer).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use GapBuffer;
    use DEFAULT_BUFFER_CAPACITY;

    const TEST_STRING: &str = r"The quick brown
    fox jumped over
    the lazy dog.";

    fn assert_bytes_eq(left: Vec<u8>, right: Vec<u8>) {
        let debug_message = format!(
            "Left: '{}'; Right: '{}'",
            std::str::from_utf8(&left).unwrap(),
            std::str::from_utf8(&right).unwrap()
        );

        assert_eq!(left, right, "{}", debug_message);
    }

    #[test]
    fn initialized_empty() {
        let buffer = GapBuffer::new();

        assert_eq!(buffer.capacity(), DEFAULT_BUFFER_CAPACITY);
        assert_eq!(buffer.len(), 0);
        assert_eq!(buffer.to_string(), "")
    }

    #[test]
    fn initialized_from_string() {
        let buffer = GapBuffer::from(TEST_STRING.to_string());

        assert_eq!(buffer.len(), TEST_STRING.len());
        assert_eq!(buffer.to_string(), TEST_STRING);
    }

    #[test]
    fn insert_into_empty_buffer() {
        let mut buffer = GapBuffer::new();
        let characters = String::from("The q").into_bytes();
        let expected_length = characters.len();

        let mut index = 0;
        for character in characters {
            buffer.insert(index, character);
            index += 1;
        }

        assert_eq!(buffer.capacity(), DEFAULT_BUFFER_CAPACITY);
        assert_eq!(buffer.len(), expected_length);
        assert_eq!(buffer.to_string(), "The q");
    }

    #[test]
    fn insert_into_buffer_at_end_of_contents() {
        let mut buffer = GapBuffer::from(TEST_STRING.to_string());
        let characters = String::from(" And the fence.");
        let expected_length = characters.len() + TEST_STRING.len();
        let expected_string = TEST_STRING.to_owned() + &characters;

        let mut index = buffer.len();
        for character in characters.into_bytes() {
            buffer.insert(index, character);
            index += 1;
        }

        assert_eq!(buffer.len(), expected_length);
        assert_eq!(buffer.to_string(), expected_string);
    }

    #[test]
    fn insert_into_buffer_within_contents() {
        let mut buffer = GapBuffer::from(TEST_STRING.to_string());
        let characters = String::from("tan ");
        let expected_length = characters.len() + TEST_STRING.len();
        let mut expected_string = TEST_STRING.to_owned();

        let mut index = 10;
        expected_string.insert_str(index, &characters);

        for character in characters.into_bytes() {
            buffer.insert(index, character);
            index += 1;
        }

        assert_eq!(buffer.len(), expected_length);
        assert_eq!(buffer.to_string(), expected_string);
    }

    #[test]
    fn remove_from_buffer() {
        let n: u8 = 0x006e;
        let mut buffer = GapBuffer::from(TEST_STRING.to_string());
        let mut expected_string = TEST_STRING.to_owned();
        expected_string.remove(14);

        assert_eq!(buffer.remove(14), n);
        assert_eq!(buffer.to_string(), expected_string);
    }

    #[test]
    fn remove_range_from_buffer() {
        let expected_bytes = "quick ".as_bytes().to_vec();
        let mut buffer = GapBuffer::from(TEST_STRING.to_string());
        let mut expected_string = TEST_STRING.to_owned();
        expected_string.drain(4..10);

        assert_bytes_eq(buffer.remove_range(4..10), expected_bytes);
        assert_eq!(buffer.to_string(), expected_string);
    }
}
