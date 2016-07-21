pub fn verse(initial: u32) -> String {
    match initial {
         2 =>
             "2 bottles of beer on the wall, 2 bottles of beer.\n\
              Take one down and pass it around, 1 bottle of beer on the wall.\n"
                .to_string(),
         1 =>
             "1 bottle of beer on the wall, 1 bottle of beer.\n\
              Take it down and pass it around, no more bottles of beer on the wall.\n"
                .to_string(),
         0 =>
             "No more bottles of beer on the wall, no more bottles of beer.\n\
              Go to the store and buy some more, 99 bottles of beer on the wall.\n"
                .to_string(),
         _ =>
             format!(
                 "{0} bottles of beer on the wall, {0} bottles of beer.\n\
                  Take one down and pass it around, {1} bottles of beer on the wall.\n",
                  initial, initial-1,
             ),
    }
}

pub fn sing(from_number: u32, to_number: u32) -> String {
    let mut song = verse(from_number).to_string();
    for number in (to_number..from_number).rev() {
        song = format!("{}\n{}", song, verse(number));
    }
    song
}
