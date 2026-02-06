/***
 * Template for an implementation of a BST.
 * Author: Dmitry Chirin <dmitryc@kth.se>
 */

use std::fs::File;
use std::io::{BufRead, BufReader};
use rstest::*;

/// An example of a function using BST to sort a vec
fn tree_sort(mut input: Vec<i32>) -> Vec<i32> {
    // Implement the sorting using a BST
    return input;
}

// Simple tests
#[rstest]
#[case(1)]
#[case(2)]
#[case(3)]
#[case(4)]
#[case(5)]
#[case(6)]
#[case(7)]
#[case(8)]
fn run_tests(#[case] test: i32) {
    // Get test file names, input and poutput
    let test_name = "tests/test_".to_owned() + &test.to_string();
    let inp = "_input.txt";
    let out = "_output.txt";
    let reader_inp = BufReader::new(File::open(test_name.to_owned() + inp).expect("Can't open file"));
    let reader_out = BufReader::new(File::open(test_name.to_owned() + out).expect("Can't open file"));

    // Read input file
    let mut unsorted: Vec<i32> = vec![];
    for line in reader_inp.lines().skip(1) {
        for word in line.unwrap().split_whitespace() {
            unsorted.push(word.parse().unwrap());
        }
    }

    // Read output file
    let mut sorted: Vec<i32> = vec![];
    for line in reader_out.lines().skip(1) {
        for word in line.unwrap().split_whitespace() {
            sorted.push(word.parse().unwrap());
        }
    }

    // Compare
    assert_eq!(tree_sort(unsorted.clone()), sorted);
}
