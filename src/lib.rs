use std::fmt;
use std::ops::Range;

const DEFAULT_BUFFER_CAPACITY: usize = 10;
const INITIAL_GAP_SIZE: usize = 10;

/// GapBuffer is a datastructure designed for efficient local insertion and deletion operations.
/// - `point`: The current index where operations are taking place.
struct GapBuffer {
    buffer: Vec<u8>,
    point: usize,
    gap_start: usize,
    gap_end: usize,
}

impl GapBuffer {
    fn new() -> GapBuffer {
        GapBuffer {
            buffer: vec![0; INITIAL_GAP_SIZE],
            point: 0,
            gap_start: 0,
            gap_end: INITIAL_GAP_SIZE,
        }
    }

    fn from(content: String) -> GapBuffer {
        let gap_bytes: [u8; INITIAL_GAP_SIZE] = [0; INITIAL_GAP_SIZE];
        let buffer_length = content.len() + gap_bytes.len();
        let mut buffer: Vec<u8> = Vec::with_capacity(buffer_length);

        let gap_start = content.len();

        for byte in content.as_bytes() {
            buffer.push(*byte);
        }

        buffer.extend(gap_bytes.iter());

        GapBuffer {
            point: gap_start,
            gap_start,
            gap_end: buffer_length,
            buffer,
        }
    }

    fn capacity(&self) -> usize {
        self.buffer.capacity()
    }

    fn len(&self) -> usize {
        let gap_length = self.gap_end - self.gap_start;
        self.buffer.len() - gap_length
    }

    /// set_point() will panic if `index` is greater than the buffer length - 1.
    fn set_point(&mut self, index: usize) {
        if index > self.len() - 1 {
            panic!("Index out of bounds. The length is {} but the index is {}.", self.len(), index)
        }

        self.point = index;
    }

    fn get_point(&self) -> usize {
        self.point
    }

    fn convert_user_index_to_gap_index(&self, index: usize) -> usize {
        if index < self.gap_start {
            index
        } else {
            (self.gap_end - self.gap_start) + index
        }
    }

    fn convert_gap_index_to_user_index(&self, index: usize) -> usize {
        if index < self.gap_start {
            index
        } else {
            index - (self.gap_end - self.gap_start)
        }
    }

    fn prepare_gap(&mut self) {
        if self.is_gap_start_before_point(){
            let quantity_characters_to_move = self.convert_user_index_to_gap_index(self.point) - self.gap_end;
            let bytes: Vec<u8> = self.buffer.drain(self.gap_end..self.gap_end + quantity_characters_to_move).collect();

            for byte in bytes {
                self.buffer.insert(self.gap_start, byte);
                self.gap_start += 1;
                self.gap_end += 1;
            }
        } else if self.is_gap_start_after_point() {
            let quantity_characters_to_move = self.gap_start - self.convert_user_index_to_gap_index(self.point);
            let bytes: Vec<u8> = self.buffer.drain(self.convert_user_index_to_gap_index(self.point)..self.gap_start).collect();

            self.gap_start -= quantity_characters_to_move;
            self.gap_end -= quantity_characters_to_move;

            let mut index = self.gap_end;
            for byte in bytes {
                self.buffer.insert(index, byte);
                index += 1;
            }
        }
    }

    fn insert(&mut self, byte: u8) {
        self.prepare_gap();
        self.gap_start += 1;
        self.buffer[self.point] = byte;
    }

    fn insert_bytes(&mut self, bytes: Vec<u8>) {
        self.prepare_gap();

        let mut index = self.point;
        for byte in bytes {
            self.buffer[index] = byte;
            index += 1;
            self.gap_start += 1;
        }
    }

    fn is_gap_start_before_point(&self) -> bool {
        self.gap_start < self.convert_user_index_to_gap_index(self.point)
    }

    fn is_gap_start_after_point(&self) -> bool {
        self.gap_start > self.convert_user_index_to_gap_index(self.point)
    }

    fn remove(&mut self) {
        self.prepare_gap();
        self.gap_start -= 1;
        self.set_point(self.point - 1)
    }

    // TODO: move the gap for insert, insert_bytes, remove, and remove_bytes.
    fn remove_bytes(&mut self, range: Range<usize>) -> Vec<u8> {
        self.buffer.drain(range).collect()
    }
}

impl fmt::Display for GapBuffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buffer_contents = std::str::from_utf8(&self.buffer).unwrap().to_owned();
        let gap_range = self.gap_start..self.gap_end;

        buffer_contents.replace_range(gap_range, "");

        write!(f, "{}", buffer_contents)
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
    fn insertion_into_full_buffer_allocates_more_capacity() {
        let mut buffer = GapBuffer::from(TEST_STRING.to_string());
        let capacity_before_insertion = buffer.capacity();
        let characters = String::from(" And the fence.");
        let expected_string = TEST_STRING.to_owned() + &characters;
        let index = buffer.len();

        buffer.set_point(index);
        buffer.insert_bytes(characters.into_bytes());

        assert!(buffer.capacity() > capacity_before_insertion);
        assert_eq!(buffer.to_string(), expected_string);
    }

    struct SingleByteTestCase {
        name: String,
        character: u8,
        index: usize,
    }

    #[test]
    fn insert_single_byte() {
        let test_cases = [
            SingleByteTestCase {
                name: "insert 's' into buffer midpoint".to_string(),
                character: 0x0073,
                index: TEST_STRING.len() / 2,
            },
            SingleByteTestCase {
                name: "insert 't' into buffer lower boundary".to_string(),
                character: 0x0074,
                index: 0,
            },
            SingleByteTestCase {
                name: "insert 'q' into buffer upper boundary".to_string(),
                character: 0x0071,
                index: TEST_STRING.len() - 1,
            },
            SingleByteTestCase {
                name: "insert 'A' into buffer lower half".to_string(),
                character: 0x0041,
                index: (TEST_STRING.len() / 2) / 2
            },
            SingleByteTestCase {
                name: "insert 'S' into buffer upper half".to_string(),
                character: 0x0053,
                index: 40,
            },
        ];

        for test_case in test_cases.iter() {
            let mut buffer = buffer_with_contents();
            let mut expected_string = TEST_STRING.to_owned();
            let byte = vec![test_case.character];
            let character = std::str::from_utf8(&byte).unwrap();
            expected_string.insert_str(test_case.index, character);

            buffer.set_point(test_case.index);
            buffer.insert(test_case.character);

            assert_eq!(buffer.to_string(), expected_string, "Test case: \"{}\" failed.", test_case.name);
        }
    }

    struct BytesInsertionTestCase {
        name: String,
        characters: String,
        index: usize,
    }

    #[test]
    fn insert_multiple_bytes() {
        let test_cases = [
            BytesInsertionTestCase {
                name: "Insert 'foxy' into buffer midpoint".to_string(),
                characters: "foxy".to_string(),
                index: TEST_STRING.len() / 2,
            },
            BytesInsertionTestCase {
                name: "Insert 'Look! ' into buffer lower boundary".to_string(),
                characters: "Look! ".to_string(),
                index: 0,
            },
            BytesInsertionTestCase {
                name: "Insert 'Yeah.' into buffer upper boundary".to_string(),
                characters: "Yeah.".to_string(),
                index: TEST_STRING.len() - 1,
            },
            BytesInsertionTestCase {
                name: "Insert 'tan' into lower half".to_string(),
                characters: "tan".to_string(),
                index: (TEST_STRING.len() / 2) / 2,
            },
            BytesInsertionTestCase {
                name: "Insert 'slow' into upper half".to_string(),
                characters: "slow".to_string(),
                index: 40,
            },
        ];

        for test_case in test_cases.iter() {
            let mut buffer = buffer_with_contents();
            let mut expected_string = TEST_STRING.to_owned();
            expected_string.insert_str(test_case.index, &test_case.characters);

            buffer.set_point(test_case.index);
            buffer.insert_bytes(test_case.characters.to_owned().into_bytes());

            assert_eq!(buffer.to_string(), expected_string, "Test case: \"{}\" failed.", test_case.name);
        }
    }

    #[test]
    fn remove_single_byte() {
        let test_cases = [
            SingleByteTestCase {
                name: "Remove 'x' from buffer midpoint".to_string(),
                character: 0x0078,
                index: 19,
            },
            SingleByteTestCase {
                name: "Remove 'T' from the buffer lower boundary".to_string(),
                character: 0x0054,
                index: 1,
            },
            SingleByteTestCase {
                name: "Remove '.' from the buffer upper boundary".to_string(),
                character: 0x002e,
                index: TEST_STRING.len() - 1,
            },
            SingleByteTestCase {
                name: "Remove 'b' from the lower half".to_string(),
                character: 0x0062,
                index: 10,
            },
            SingleByteTestCase {
                name: "Remove 'e' from the upper half".to_string(),
                character: 0x0065,
                index: 34,
            },
        ];

        for test_case in test_cases.iter() {
            let mut buffer = buffer_with_contents();
            let mut expected_string = TEST_STRING.to_owned();
            expected_string.remove(test_case.index - 1);

            buffer.set_point(test_case.index);
            buffer.remove();

            assert_eq!(buffer.to_string(), expected_string, "Test case: \"{}\" failed.", test_case.name);
            assert_eq!(buffer.get_point(), test_case.index - 1, "Test case: \"{}\" failed. Point not at index {}", test_case.name, test_case.index - 1);
        }
    }

    #[test]
    fn remove_bytes_from_buffer() {
        let expected_bytes = "quick ".as_bytes().to_vec();
        let mut buffer = GapBuffer::from(TEST_STRING.to_string());
        let mut expected_string = TEST_STRING.to_owned();
        expected_string.drain(4..10);

        assert_bytes_eq(buffer.remove_bytes(4..10), expected_bytes);
        assert_eq!(buffer.to_string(), expected_string);
    }

    #[test]
    fn set_the_point() {
        let mut buffer = GapBuffer::from(TEST_STRING.to_string());

        buffer.set_point(8);
        assert_eq!(buffer.get_point(), 8);

        buffer.set_point(44);
        assert_eq!(buffer.get_point(), 44);

        buffer.set_point(0);
        assert_eq!(buffer.get_point(), 0);
    }

    #[test]
    #[should_panic(expected = "Index out of bounds. The length is 45 but the index is 50.")]
    fn set_the_point_out_of_bounds_panics() {
        let mut buffer = GapBuffer::from(TEST_STRING.to_string());

        buffer.set_point(50);
    }

    fn buffer_with_contents() -> GapBuffer {
        GapBuffer::from(TEST_STRING.to_string())
    }
}
