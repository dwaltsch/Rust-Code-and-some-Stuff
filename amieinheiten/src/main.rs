use std::io;

fn main() {
    println!("How much Fahreinheit");
loop {
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };
    println!("That is {} degrees of freedom", (guess - 30)/2);
}
}
