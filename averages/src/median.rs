use std::vec::Vec;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn odds() {
    let vec = vec![5, 4, 2, 1, 3];
    let median = get_median(&vec);
    let result = match median {
      Values::S(single) => single,
      _ => panic!(),
    };
    assert_eq!(result, 3);
  }

  #[test]
  fn evens() {
    let vec = vec![5, 4, 2, 1, 3, 4];
    let result = match get_median(&vec) {
      Values::T(i1, i2) => (i1, i2),
      _ => panic!(),
    };
    assert_eq!(result, (3, 4));
  }
}

#[derive(Debug)]
pub enum Values {
  S(i32),
  T(i32, i32),
}

pub fn get_median(vec: &Vec<i32>) -> Values {
  let mut sorted_vec = vec.to_vec();
  sorted_vec.sort_unstable();
  let len = sorted_vec.len();
  let is_even = len % 2 == 0;
  match is_even {
    true => Values::T(sorted_vec[len / 2 - 1], sorted_vec[len / 2]),
    false => Values::S(sorted_vec[len / 2]),
  }
}
