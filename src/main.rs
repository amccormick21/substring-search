use std::{collections::{HashMap, VecDeque}, env};

fn main() {
    let args: Vec<String> = env::args().collect();

    let s = &args[1];
    let words: Vec<&str> = args[2].split(",").collect();

    println!("Input string : {}", s);
    println!("Input words : {:?}", words);
    println!(
        "The count of words which exist as substrings in s: {}",
        count_substrings_impl(&s, &words, linked_list_solver_wrapper)
    );
    println!(
        "The count of words which exist as substrings in s: {}",
        count_substrings_impl(&s, &words, two_pointer_solver)
    );
    println!(
        "The count of words which exist as substrings in s: {}",
        count_substrings_impl(&s, &words, hashmap_solver)
    );
}

fn linked_list_solver_wrapper(s: &str, word: &str) -> bool {
    if s.len() < word.len() {
        return false;
    }
    if s.len() == 0 {
        return false;
    }

    let mut s_chars = s.chars();
    let mut word_chars = word.chars();

    linked_list_solver(&mut s_chars, &mut word_chars)
}

fn linked_list_solver(s_chars: &mut std::str::Chars, word_chars: &mut std::str::Chars) -> bool {
    match word_chars.nth(0) {
        None => true,
        Some(first_substring_char) => {
            while let Some(indexed_s_char) = s_chars.next() {
                if indexed_s_char == first_substring_char {
                    return linked_list_solver(s_chars, word_chars);
                }
            }
            false
        }
    }
}

struct TwoPointerStringSlice<'a> {
    s: &'a str,
    left: usize,
    right: usize,
}

impl<'a> TwoPointerStringSlice<'a> {
    fn new(s: &'a str) -> Result<Self, ()> {
        match s.chars().count() {
            0 => Err(()),
            _ => Ok(Self {
                s,
                left: 0,
                right: s.chars().count() - 1,
            }),
        }
    }

    fn left(&self) -> Option<char> {
        self.s.chars().nth(self.left)
    }

    fn right(&self) -> Option<char> {
        self.s.chars().nth(self.right)
    }

    fn length(&self) -> usize {
        self.right - self.left + 1
    }

    fn next_left(&mut self) -> Option<char> {
        // Is it safe to call this?
        if self.is_complete() {
            return None;
        }
        // Return the value at the current left pointer, then increment
        let ret = self.left();
        self.left += 1;
        ret
    }

    fn next_right(&mut self) -> Option<char> {
        // Is it safe to call this?
        if self.is_complete() {
            return None;
        }
        // Return the value at the current left pointer, then increment
        let ret = self.right();
        self.right -= 1;
        ret
    }

    fn substring(&self) -> &str {
        &self.s[self.left..self.right]
    }

    fn is_complete(&mut self) -> bool {
        self.left > self.right
    }
}

fn two_pointer_solver(s: &str, word: &str) -> bool {
    if s.len() < word.len() {
        return false;
    }
    let s_slice_result = TwoPointerStringSlice::new(s);
    let word_slice_result = TwoPointerStringSlice::new(word);

    if let Err(_) = s_slice_result {
        return false;
    }  
    if let Err(_) = word_slice_result {
        return true;
    }

    // Unwraps are safe because the zero-length is already handled
    let mut s_slice = s_slice_result.unwrap();
    let mut word_slice = word_slice_result.unwrap();


    while !s_slice.is_complete() {
        if let Some(s_left) = s_slice.next_left() {
            if let Some(word_left) = word_slice.left() {
                if s_left == word_left {
                    word_slice.next_left();
                }
            }
        }

        if word_slice.is_complete() {
            return true;
        }
    }
    false
}

fn hashmap_solver(s: &str, word: &str) -> bool {

    let mut s_hashmap: HashMap<char, VecDeque<usize>> = HashMap::new();
    s.chars().enumerate().for_each(|(i, s_char)| {
        match s_hashmap.get_mut(&s_char) {
            Some(indices) => {indices.push_back(i); },
            None => {s_hashmap.insert(s_char, VecDeque::from([i])); },
        }
    });

    let mut s_position: usize = 0;
    let word_indices: Vec<Option<usize>> = word.chars().map(|w_char| -> Option<usize> {
        match s_hashmap.get_mut(&w_char) {
            Some(indices) => {
                let position_in_s = indices.iter().enumerate().find(|(_i, index)| { index >= &&s_position });
                match position_in_s {
                    Some((i, &index)) => {
                        s_position = index;
                        indices.remove(i);
                        Some(index)
                    },
                    None => None,
                }
            }
            None => None,
        }
    }).collect::<Vec<Option<usize>>>();

    if word_indices.iter().any(|index_option| index_option.is_none()) {
        return false;
    }
    
    word_indices.iter().is_sorted()
}

fn count_substrings_impl<T>(s: &str, words: &[&str], solver: T) -> usize
where
    T: Fn(&str, &str) -> bool,
{
    words.iter().filter(|&w| solver(s, w)).count()
}

#[cfg(test)]
mod test {

    use super::*;

    struct TestData {
        pub s: &'static str,
        pub words: Vec<&'static str>,
        pub expected: usize,
    }

    impl TestData {
        fn new(s: &'static str, words: Vec<&'static str>, expected: usize) -> Self {
            Self { s, words, expected }
        }
    }

    fn test_data_basic() -> TestData {
        TestData::new("abcde", vec!["a", "bb", "acd", "ace"], 3)
    }

    fn test_data_random() -> TestData {
        TestData::new(
            "dsahjpjauf",
            vec!["ahjpjau", "ja", "ahbwzgqnuk", "tnmlanowax"],
            2,
        )
    }

    fn test_data_empty() -> TestData {
        TestData::new("", vec!["a", "b"], 0)
    }

    fn test_data_empty_words() -> TestData {
        TestData::new("abc", vec!["", ""], 2)
    }

    fn test_data_repeated() -> TestData {
        TestData::new("aaaaaaaaaa", vec!["a", "aa", "aaa"], 3)
    }

    fn test_data_long() -> TestData {
        TestData::new(
            "thequickbrownfoxjumpsoverthelazydog",
            vec!["the", "quick", "fox", "dog", "cat"],
            4,
        )
    }

    #[test]
    fn test_count_substrings_linked_list_basic() {
        let test_data = test_data_basic();
        let result =
            count_substrings_impl(&test_data.s, &test_data.words, linked_list_solver_wrapper);
        assert_eq!(result, test_data.expected);
    }

    #[test]
    fn test_count_substrings_linked_list_random() {
        let test_data = test_data_random();
        let result =
            count_substrings_impl(&test_data.s, &test_data.words, linked_list_solver_wrapper);
        assert_eq!(result, test_data.expected);
    }

    #[test]
    fn test_count_substrings_linked_list_empty() {
        let test_data = test_data_empty();
        let result =
            count_substrings_impl(&test_data.s, &test_data.words, linked_list_solver_wrapper);
        assert_eq!(result, test_data.expected);
    }

    #[test]
    fn test_count_substrings_linked_list_empty_words() {
        let test_data = test_data_empty_words();
        let result =
            count_substrings_impl(&test_data.s, &test_data.words, linked_list_solver_wrapper);
        assert_eq!(result, test_data.expected);
    }

    #[test]
    fn test_count_substrings_linked_list_repeated() {
        let test_data = test_data_repeated();
        let result =
            count_substrings_impl(&test_data.s, &test_data.words, linked_list_solver_wrapper);
        assert_eq!(result, test_data.expected);
    }

    #[test]
    fn test_count_substrings_linked_list_long() {
        let test_data = test_data_long();
        let result =
            count_substrings_impl(&test_data.s, &test_data.words, linked_list_solver_wrapper);
        assert_eq!(result, test_data.expected);
    }

    #[test]
    fn test_count_substrings_two_pointer_basic() {
        let test_data = test_data_basic();
        let result = count_substrings_impl(&test_data.s, &test_data.words, two_pointer_solver);
        assert_eq!(result, test_data.expected);
    }

    #[test]
    fn test_count_substrings_two_pointer_random() {
        let test_data = test_data_random();
        let result = count_substrings_impl(&test_data.s, &test_data.words, two_pointer_solver);
        assert_eq!(result, test_data.expected);
    }

    #[test]
    fn test_count_substrings_two_pointer_empty() {
        let test_data = test_data_empty();
        let result = count_substrings_impl(&test_data.s, &test_data.words, two_pointer_solver);
        assert_eq!(result, test_data.expected);
    }

    #[test]
    fn test_count_substrings_two_pointer_empty_words() {
        let test_data = test_data_empty_words();
        let result = count_substrings_impl(&test_data.s, &test_data.words, two_pointer_solver);
        assert_eq!(result, test_data.expected);
    }

    #[test]
    fn test_count_substrings_two_pointer_repeated() {
        let test_data = test_data_repeated();
        let result = count_substrings_impl(&test_data.s, &test_data.words, two_pointer_solver);
        assert_eq!(result, test_data.expected);
    }

    #[test]
    fn test_count_substrings_two_pointer_long() {
        let test_data = test_data_long();
        let result = count_substrings_impl(&test_data.s, &test_data.words, two_pointer_solver);
        assert_eq!(result, test_data.expected);
    }

    #[test]
    fn test_count_substrings_hashmap_basic() {
        let test_data = test_data_basic();
        let result = count_substrings_impl(&test_data.s, &test_data.words, hashmap_solver);
        assert_eq!(result, test_data.expected);
    }

    #[test]
    fn test_count_substrings_hashmap_random() {
        let test_data = test_data_random();
        let result = count_substrings_impl(&test_data.s, &test_data.words, hashmap_solver);
        assert_eq!(result, test_data.expected);
    }

    #[test]
    fn test_count_substrings_hashmap_empty() {
        let test_data = test_data_empty();
        let result = count_substrings_impl(&test_data.s, &test_data.words, hashmap_solver);
        assert_eq!(result, test_data.expected);
    }

    #[test]
    fn test_count_substrings_hashmap_empty_words() {
        let test_data = test_data_empty_words();
        let result = count_substrings_impl(&test_data.s, &test_data.words, hashmap_solver);
        assert_eq!(result, test_data.expected);
    }

    #[test]
    fn test_count_substrings_hashmap_repeated() {
        let test_data = test_data_repeated();
        let result = count_substrings_impl(&test_data.s, &test_data.words, hashmap_solver);
        assert_eq!(result, test_data.expected);
    }

    #[test]
    fn test_count_substrings_hashmap_long() {
        let test_data = test_data_long();
        let result = count_substrings_impl(&test_data.s, &test_data.words, hashmap_solver);
        assert_eq!(result, test_data.expected);
    }
}
