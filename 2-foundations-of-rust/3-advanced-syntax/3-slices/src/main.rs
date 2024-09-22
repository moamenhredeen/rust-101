// This a unfinished implementation of the well-known merge sort algorithm
// 1. Fix the language problems in the function merge
// 2. Finish the implementation of the function merge_sort
// 3. EXTRA: try changing the type from i32 into String everywhere; does your program still compile? What changes are necessary?

use std::io::{self, BufRead};

// merge two sorted slices
fn merge(left: &[i32], right: &[i32]) -> Vec<i32> {
    let mut dest = Vec::new();

    let mut l_i = 0;
    let mut r_i = 0;

    while l_i < left.len() && r_i < right.len() {
        if left[l_i] <= right[r_i] {
            dest.push(left[l_i]);
            l_i += 1
        } else {
            dest.push(right[r_i]);
            r_i += 1
        }
    }

    for elem in &left[l_i..] {
        dest.push(*elem)
    }
    for elem in &right[r_i..] {
        dest.push(*elem)
    }

    dest
}

// sort merge a slice
fn merge_sort(data: &[i32]) -> Vec<i32> {
    if data.len() > 1 {
        let pivot: usize = data.len() / 2;
        let left = &data[..pivot];
        let right = &data[pivot..];
        let sorted_left = merge_sort(left);
        let sorted_right = merge_sort(right);
        merge(&sorted_left, &sorted_right)
    } else {
        data.to_vec()
    }
}

/// Read a bunch of numbers from standard input into a Vec<i32>.
fn read_numbers() -> Result<Vec<i32>, io::Error> {
    use std::io;
    let mut result = Vec::new();
    let mut line = String::new();
    io::stdin().lock().read_line(&mut line)?;
    for word in line.split_whitespace() {
        result.push(word.parse().unwrap())
    }
    Ok(result)
}

fn main() {
    let input = read_numbers().unwrap();
    println!("Data to be sorted:");
    println!("{input:?}");

    let sorted_input = merge_sort(&input);
    println!("Sorted data:");
    println!("{sorted_input:?}");
}

// you can run these automatic tests by typing 'cargo test'
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sort() {
        assert_eq!(merge_sort(&[]), vec![]);
        assert_eq!(merge_sort(&[5]), vec![5]);
        assert_eq!(merge_sort(&[1, 2, 3]), vec![1, 2, 3]);
        assert_eq!(merge_sort(&[47, 42, 5, 1]), vec![1, 5, 42, 47]);
        assert_eq!(
            merge_sort(&[6, 47, 42, 5, 1, 123]),
            vec![1, 5, 6, 42, 47, 123]
        );
    }
}
