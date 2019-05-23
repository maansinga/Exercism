pub fn verse(n: i32) -> String {
    match n{
        0 => "No more bottles of beer on the wall, no more bottles of beer.\n\
        Go to the store and buy some more, 99 bottles of beer on the wall.\n".to_string(),

        1 => "1 bottle of beer on the wall, 1 bottle of beer.\n\
        Take it down and pass it around, no more bottles of beer on the wall.\n".to_string(),

        2 => "2 bottles of beer on the wall, 2 bottles of beer.\n\
        Take one down and pass it around, 1 bottle of beer on the wall.\n".to_string(),

        x if x >= 3 => format!("{} bottles of beer on the wall, {} bottles of beer.\n\
        Take one down and pass it around, {} bottles of beer on the wall.\n", x, x, x-1),

         _ => format!("This can't happen!"),
    }
}

pub fn sing(start: i32, end: i32) -> String {
    let mut output: String = "".to_string();

    for k in ((end+1)..=start).rev(){
        let v = verse(k);

        output.push_str(&v);
        output.push_str("\n");
    }

    output.push_str(&verse(end));
    output
}
