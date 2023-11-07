pub fn verse(n: u32) -> String {
    let verse: String = match n {
        n if n == 0 => format!("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n"),
        n if n == 1 => format!("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n"),
        n if n == 2 => format!("2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n"),
        n => format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n",n,n,n-1),
    };
    return String::from(verse);
}

pub fn sing(start: u32, end: u32) -> String {
    let mut song: String = String::new();
    for i in (end..=start).rev() {
        song.push_str(&verse(i));
        if i != end {
            song.push_str("\n");
        }
    }
    song
}
