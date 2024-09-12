use std::io;

fn main() {
  println!("Time to guess the number! (0-10)");

  println!("Please input a guess: \n");

  let mut guess = String::New();

  io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");
  println!("You guessed: {}", guess);
}
