pub fn verse(initial: u32) -> String {
    match initial {
        99 => verse_for(99, 98),
         1 => verse_for( 1,  0),
         0 => verse_for( 0, 99),
         _ => verse_for(initial, initial-1),
    }
}

pub fn sing(from_number: u32, to_number: u32) -> String {
    let mut song = verse(from_number).to_string();
    for number in (to_number..from_number).rev() {
        song = format!("{}\n{}", song, verse(number));
    }
    song
}

fn verse_for(initial: u32, next: u32) -> String {
    let initial_phrase = phrase_for(initial);
    let action = action_for(initial);
    let next_phrase = phrase_for(next);
    capitalize(&format!("{} of beer on the wall, {} of beer.\n\
        {}, {} of beer on the wall.\n\
        ", initial_phrase, initial_phrase, action, next_phrase))
}

fn phrase_for(number: u32) -> String {
    match number {
        0 => "no more bottles".to_string(),
        1 => "1 bottle".to_string(),
        _ => format!("{} bottles", number),
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
    match number {
        0 => "Go to the store and buy some more",
        1 => "Take it down and pass it around",
        _ => "Take one down and pass it around",
    }
}
