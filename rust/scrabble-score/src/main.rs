use scrabble_score::score;

fn main(){
    let word = "SAM"; //1 + 1 + 3 = 5
    println!("{} {}", word, score(word));
}