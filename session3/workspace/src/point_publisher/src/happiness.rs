pub fn is_happy(message: &str) -> bool {
  match &message.to_lowercase()[..] {
      "c++" => false,
      "french" => false,
      _ => true
  }
}