
use std::io;
fn main() {

    let mut og_str = String::new();
    let mut new_str = String::new();

    let vowels = ['a', 'e', 'i', 'o', 'u'];
    io::stdin().read_line(&mut og_str).ok().expect("read error");
    og_str = og_str.to_lowercase();

    //https://stackoverflow.com/questions/30811107/getting-a-single-character-out-of-a-string
    let first_char = og_str.chars().next().unwrap();

    if vowels.contains(&first_char) {
        new_str = format!("{}-hay", &og_str);
    } else {
        for (i, c) in og_str.char_indices() {
            if i == 0 {
                continue;
            } else {
                new_str.push(c);
            }
        }
        new_str = format!("{}-{}ay", new_str, first_char);

    }

    println!("{}", new_str);
}
