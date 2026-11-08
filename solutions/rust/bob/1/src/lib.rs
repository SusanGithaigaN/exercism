// use itertools::{Itertools, Position};

pub fn reply(message: &str) -> &str {
    // todo!("have Bob reply to the incoming message: {message}")
    let message = message.trim();
    let is_yelling = message.chars().any(|c| c.is_alphabetic()) && message == message.to_uppercase();
    
    if message.is_empty() {
        return "Fine. Be that way!";
    } else if message.ends_with("?") && is_yelling {
        return "Calm down, I know what I'm doing!";
    } else if message.ends_with("?") {
        return "Sure.";
    } else if is_yelling {
        return "Whoa, chill out!";
    } else {
        return "Whatever.";
    }
}
