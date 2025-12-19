# Substring Search

This repository demonstrates several different approaches in Rust for solving the ordered substring counting problem: determining how many words from a given list appear as ordered (but non-continuous) substrings within a starting string.

## Problem Description

Given a starting string and a list of words, count how many of those words can be formed by selecting characters from the starting string in order, but not necessarily consecutively.

For example, if the starting string is `"abppplee"` and we're checking the word `"apple"`:
- We can find `a` at position 0
- We can find `p` at position 2
- We can find `p` at position 3
- We can find `l` at position 5
- We can find `e` at position 6

Therefore, `"apple"` is an ordered substring of `"abppplee"`.

## Background

This problem is based on an interview question covered in this YouTube video:  
https://www.youtube.com/watch?v=Ebyesd3mPAA

## Test Cases

### Basic Test Case
**Starting string:** `"abcde"`  
**Words list:** `["a", "bb", "acd", "ace"]`  
**Expected result:** `3`

## Running the Code

### Run with arguments:
```bash
cargo run --release <starting_string> <word1>,<word2>,<word3>
```

Example:
```bash
cargo run --release "abppplee" "able,ale,apple,bale,kangaroo"
```

### Run tests:
```bash
cargo test
```

### Run tests with output:
```bash
cargo test -- --nocapture
```

## Implementation Approaches

This repository contains multiple Rust implementations demonstrating different algorithmic approaches to solve this problem, including variations in efficiency and code style.
