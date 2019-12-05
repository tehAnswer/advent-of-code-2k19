pub fn digits(formatted_number: String) -> Vec<i32> {
  formatted_number
    .chars()
    .map(|c| c.to_digit(10).unwrap() as i32)
    .collect()
}
