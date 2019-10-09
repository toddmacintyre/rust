use std::i32;
use std::vec::Vec;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let vec = vec![1, 2, 3, 8, 9, 10];
    let mean = get_mean(&vec);
    assert_eq!(mean, 5.5);
  }

  // not specific to mean. just testing out.
  #[test]
  fn raise_to_powers() {
    assert_eq!(2_f64.powi(3), 8_f64);
    assert_eq!(2_f64.powi(0), 1_f64);
  }
}

pub fn get_mean(vec: &Vec<i32>) -> f64 {
  let len = vec.len() as i32;
  let mut sum: i32 = 0;
  for num in vec {
    sum += *num;
  }

  let amount_to_round = 10_f64.powi(2);
  (sum as f64 / len as f64 * amount_to_round).round() / amount_to_round
}
