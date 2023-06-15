use std::io;

struct Pig {
    words: String,
}

impl Pig {
    fn translate_to(&self) {
        for word in in &self.words.split_whitespace() {

        } 
    }
}

fn main() {
    println!("Введите строку:");
    
    let mut pig_latin = String::new();

    io::stdin()
        .read_line(&mut pig_latin)
        .expect("Failed to readline");

}