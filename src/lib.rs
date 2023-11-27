// =============================================================================
//! - CroftSoft String Library
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-11-27
//! - Updated: 2023-11-27
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use std::iter::Rev;
use std::str::Chars;

pub fn to_comma_separated(value: u64) -> String {
  let value_as_string: String = value.to_string();
  let reversed_without_commas: Rev<Chars> = value_as_string.chars().rev();
  let mut reversed_with_commas: String = "".to_string();
  for (i, c) in reversed_without_commas.enumerate() {
    if (i > 0) && (i % 3 == 0) {
      reversed_with_commas.push(',');
    }
    reversed_with_commas.push(c);
  }
  let comma_separated: String = to_reverse_string(reversed_with_commas);
  comma_separated
}

pub fn to_dollars(amount: f64) -> String {
  let rounded_amount: f64 = amount.round();
  let integer_amount: i64 = rounded_amount as i64;
  let positive_amount: u64 = integer_amount.unsigned_abs();
  let comma_separated_string: String = to_comma_separated(positive_amount);
  let mut dollars: String = "".to_owned();
  if integer_amount.is_negative() {
    dollars.push('-');
  }
  dollars.push('$');
  dollars += &comma_separated_string;
  dollars
}

pub fn to_reverse_string(s: String) -> String {
  s.chars().rev().collect::<String>()
}
