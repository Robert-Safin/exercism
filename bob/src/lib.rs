pub fn reply(message: &str) -> &str {
    if message.chars().all(|c| c.is_whitespace()) {
        "Fine. Be that way!"
    }

    else if message.trim().ends_with("?") && message == message.to_uppercase() {
        "Calm down, I know what I'm doing!"
    }

    else if message == message.to_uppercase() {
        "Whoa, chill out!"
    }

    else if message.trim().ends_with("?") {
        "Sure."
    }

    else {
        "Whatever."
    }
}
