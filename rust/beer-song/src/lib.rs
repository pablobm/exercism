pub fn verse(initial: u32) -> String {
    let next = next_number(initial);
    let initial_phrase = phrase_for(initial);
    let capitalized_initial_phrase = capitalize(&initial_phrase);
    let action = action_for(initial);
    let next_phrase = phrase_for(next);
    format!("{} of beer on the wall, {} of beer.\n\
        {}, {} of beer on the wall.\n\
        ", capitalized_initial_phrase, initial_phrase, action, next_phrase)
}

pub fn sing(from_number: u32, to_number: u32) -> String {
    let mut song = verse(from_number).to_string();
    for number in (to_number..from_number).rev() {
        song = format!("{}\n{}", song, verse(number));
    }
    song
}

fn next_number(number: u32) -> u32 {
    if number == 0 {
        99
    }
    else {
        number - 1
    }
}

fn phrase_for(number: u32) -> String {
    if number == 0 {
        "no more bottles".to_string()
    }
    else if number == 1 {
        "1 bottle".to_string()
    }
    else {
        format!("{} bottles", number)
    }
}

fn capitalize(sentence: &String) -> String {
    let mut chars = sentence.chars();
    let head = chars.next();

    let capital = match head {
        Some(c) => c.to_uppercase().next().unwrap().to_string(),
        None    => "".to_string(),
    };

    format!("{}{}", capital, chars.collect::<String>())
}

fn action_for(number: u32) -> &'static str {
    if number == 0 {
        "Go to the store and buy some more"
    }
    else if number == 1{
        "Take it down and pass it around"
    }
    else {
        "Take one down and pass it around"
    }
}
