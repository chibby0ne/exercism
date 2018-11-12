fn is_yell(message: &str) -> bool {
    // needed to check agains lowercase in order to ignore non-word characters sentences return
    // is_yell (false positives is_yells)
    return message.to_uppercase() == message && message.to_lowercase() != message;
}

fn is_question(message: &str) -> bool {
    return message.ends_with("?");
}

pub fn reply(message: &str) -> &str {
    let msg = message.trim();

    if is_yell(msg) && is_question(msg) {
        return "Calm down, I know what I'm doing!";
    } else if is_yell(msg) {
        return "Whoa, chill out!";
    } else if is_question(msg) {
        return "Sure.";
    } else if msg.len() == 0 {
        return "Fine. Be that way!";
    } else {
        return "Whatever.";
    }
}
