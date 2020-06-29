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

    // BUG: length should be how many items the buffer contains. This does not inclue the gap which is invisible to the user.
    // length should work with the user coordinates.
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

    // TODO: move the gap for insert, insert_bytes, remove, and remove_bytes.
    fn insert(&mut self, byte: u8) {
        // TODO: More tests for point at gap, point before gap, point after gap, point boundary cases.
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

        self.gap_start += 1;
        self.buffer[self.point] = byte;
    }

    fn is_gap_start_before_point(&self) -> bool {
        self.gap_start < self.convert_user_index_to_gap_index(self.point)
    }

    fn is_gap_start_after_point(&self) -> bool {
        self.gap_start > self.convert_user_index_to_gap_index(self.point)
    }

    // TODO: move the gap for insert, insert_bytes, remove, and remove_bytes.
    fn insert_bytes(&mut self, mut index: usize, bytes: Vec<u8>) {
        for byte in bytes {
            self.buffer.insert(index, byte);
            index += 1;
        }

        self.gap_start = index;
    }

    // TODO: move the gap for insert, insert_bytes, remove, and remove_bytes.
    fn remove(&mut self, index: usize) -> u8 {
        self.buffer.remove(index)
    }

    // TODO: move the gap for insert, insert_bytes, remove, and remove_bytes.
    fn remove_bytes(&mut self, range: Range<usize>) -> Vec<u8> {
        self.buffer.drain(range).collect()
    }
}

impl fmt::Display for GapBuffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buffer_contents = std::str::from_utf8(&self.buffer).unwrap().to_owned();

        // BUG: When initialized empty adding 1 onto the end of this range is too much.
        // When the gap is at the end of the buffer it is not enough to include the last gap element.
        // During insertion is the gap start being moved forward?
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

        buffer.insert_bytes(index, characters.into_bytes());

        assert!(buffer.capacity() > capacity_before_insertion);
        assert_eq!(buffer.to_string(), expected_string);
    }

    struct InsertionTestCase {
        test_name: String,
        character: u8,
        insertion_index: usize,
    }

    #[test]
    fn insert_single_byte() {
        let test_cases = [
            InsertionTestCase {
                test_name: "insert 's' into buffer midpoint".to_string(),
                character: 0x0073,
                insertion_index: TEST_STRING.len() / 2,
            },
            InsertionTestCase {
                test_name: "insert 't' into buffer lower boundary".to_string(),
                character: 0x0074,
                insertion_index: 0,
            },
            InsertionTestCase {
                test_name: "insert 'q' into buffer upper boundary".to_string(),
                character: 0x0071,
                insertion_index: TEST_STRING.len() - 1,
            },
            InsertionTestCase {
                test_name: "insert 'A' into buffer lower half".to_string(),
                character: 0x0041,
                insertion_index: (TEST_STRING.len() / 2) / 2
            },
            InsertionTestCase {
                test_name: "insert 'S' into buffer upper half".to_string(),
                character: 0x0053,
                insertion_index: 40,
            },
        ];

        for test_case in test_cases.iter() {
            let mut buffer = buffer_with_contents();
            let mut expected_string = TEST_STRING.to_owned();
            let byte = vec![test_case.character];
            let character = std::str::from_utf8(&byte).unwrap();
            expected_string.insert_str(test_case.insertion_index, character);

            buffer.set_point(test_case.insertion_index);
            buffer.insert(test_case.character);

            assert_eq!(buffer.to_string(), expected_string, "Test case: \"{}\" failed.", test_case.test_name);
        }
    }

    #[test]
    fn insert_bytes_into_buffer() {
        let mut buffer = GapBuffer::from(TEST_STRING.to_string());
        let characters = String::from("tan ");
        let index = 10;
        let mut expected_string = TEST_STRING.to_owned();
        expected_string.insert_str(index, &characters);

        buffer.insert_bytes(index, characters.into_bytes());

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
