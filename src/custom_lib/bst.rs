//
// bst.rs
// Copyright (C) 2020 Sushil Chhetri <chhetrisushil@gmail.com>
// Distributed under terms of the MIT license.
//

pub fn run(number_to_find: i32, arr: &[i32], start: i32, end: i32) -> i32 {
  if end >= start {
    let mid: i32 = (start + end) / 2;

    if arr[mid as usize] > number_to_find {
      return run(number_to_find, arr, start, mid - 1);
    }

    if arr[mid as usize] < number_to_find {
      return run(number_to_find, arr, mid + 1, end);
    }

    if arr[mid as usize] == number_to_find {
      return mid;
    }
  }

  return -1;
}
