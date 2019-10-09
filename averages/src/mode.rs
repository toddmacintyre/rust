use std::collections::HashMap;
use std::vec::Vec;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn single_value() {
    let vec = vec![1, 2, 3, 4, 4, 5];
    assert_eq!(get_mode(&vec), vec![4]);
  }

  #[test]
  fn multiple_values() {
    let vec = vec![1, 2, 2, 3, 4, 4, 5];
    assert_eq!(get_mode(&vec), vec![2, 4]);

    let vec = vec![1, 7, 7, 2, 2, 3, 4, 4, 5, 5, 6];
    assert_eq!(get_mode(&vec), vec![2, 4, 5, 7]);
  }
}

#[derive(Debug)]
struct HighestFrequency {
  value: Vec<i32>,
  frequency: i32,
}

pub fn get_mode(vec: &Vec<i32>) -> Vec<i32> {
  let mut map = HashMap::new();
  for (_, item) in vec.iter().enumerate() {
    let val = map.entry(item).or_insert(0);
    *val += 1;
  }

  let mut most_frequent = HighestFrequency {
    value: vec![vec[0]],
    frequency: 1,
  };
  for (value, frequency) in map.iter_mut() {
    if frequency > &mut most_frequent.frequency {
      most_frequent = HighestFrequency {
        value: vec![**value],
        frequency: *frequency,
      };
    } else if *frequency == most_frequent.frequency {
      most_frequent.value.push(**value);
    }
  }

  most_frequent.value.sort_unstable();
  most_frequent.value
}
