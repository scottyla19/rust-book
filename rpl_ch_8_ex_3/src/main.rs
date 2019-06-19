use std::collections::HashMap;
use std::io;
fn main() {

    let mut words = String::new();

    io::stdin().read_line(&mut words).ok().expect("read error");

    let words: Vec<&str> = words.split_whitespace().collect();

    let mut people = HashMap::new();

    let v1: Vec<String> = vec![String::from("Joe")];
    people.insert(String::from("Dev"), v1);

    let v2: Vec<String> = vec![String::from("Jill")];
    people.insert(String::from("HR"), v2);

    if words[0] == "Add" && words[2] == "to" {
        let dept = match people.get(words[3]) {
            None => None,
            Some(w) => Some(w),
        };
        if dept == None {
            let v: Vec<String> = vec![words[1].to_string()];
            people.insert(String::from(words[3]), v);
        // people.entry(String::from(words[3])).or_insert(v);
        } else {
            let mut v = dept.unwrap().to_vec();
            v.push(String::from(words[1]));
            people.insert(String::from(words[3]), v);
        }
    }
    println!("words: {:?}", words);

    println!("people: {:?}", people);
}
