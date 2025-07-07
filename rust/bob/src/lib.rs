pub fn reply(message: &str) -> &str {
    let trimmed_message = message.trim();

    if trimmed_message.is_empty() {
        return "Fine. Be that way!";
    }

    let is_question = trimmed_message.ends_with('?');
    let is_yelling =
        trimmed_message.chars().any(|c| c.is_alphabetic()) && trimmed_message.to_uppercase() == trimmed_message;

    match (is_question, is_yelling) {
        (true, true) => "Calm down, I know what I'm doing!",
        (true, false) => "Sure.",
        (false, true) => "Whoa, chill out!",
        (false, false) => "Whatever.",
    }
}
