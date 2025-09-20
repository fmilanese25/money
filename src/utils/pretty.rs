use colored::*;

pub fn pretty(msg: &str) -> String {
  let mut result = String::new();
  let mut chars = msg.chars().peekable();

  if msg.starts_with("error:") {
    result.push_str(&"error:".red().bold().to_string());
    for _ in 0.."error:".len() {
      chars.next();
    }
  } else if msg.starts_with("warning:") {
    result.push_str(&"warning:".yellow().bold().to_string());
    for _ in 0.."warning:".len() {
      chars.next();
    }
  } else if msg.starts_with("info:") {
    result.push_str(&"info:".bright_blue().bold().to_string());
    for _ in 0.."info:".len() {
      chars.next();
    }
  } else if msg.starts_with("note:") {
    result.push_str(&"note:".white().bold().to_string());
    for _ in 0.."note:".len() {
      chars.next();
    }
  } else if msg.starts_with("     Running at") {
    result.push_str(&"     Running at".green().bold().to_string());
    for _ in 0.."     Running at".len() {
      chars.next();
    }
  }

  let mut bold = false;
  while let Some(c) = chars.next() {
    if c == '*' {
      bold = !bold;
      continue; // skip the asterisk
    }

    if bold {
      result.push_str(&c.to_string().bold().to_string());
    } else {
      result.push(c);
    }
  }

  let mut bold = false;
  while let Some(c) = chars.next() {
    if c == '*' {
      bold = !bold;
      continue; // skip the asterisk
    }

    if bold {
      result.push_str(&c.to_string().bold().to_string());
    } else {
      result.push(c);
    }
  }

  let mut bold = false;
  while let Some(c) = chars.next() {
    if c == '*' {
      bold = !bold;
      continue; // skip the asterisk
    }

    if bold {
      result.push_str(&c.to_string().bold().to_string());
    } else {
      result.push(c);
    }
  }

  result
}