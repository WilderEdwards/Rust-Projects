use std::io;

fn main() {
  
  println!("Time to guess the number! (0-10)");

  println!("Please input a guess: \n");
  //create a variable to store user input
  let mut guess = String::New();
  // mutable
  //:: indicates New is an associated function of the String type, or a function implemented ON a type

  //receiving user input with namespace stdin()
  //could be used without (use std::io), with std::io::stdin
  io::stdin()
    .read_line(&mut guess)   //calls read_line method on the std handle, uses guess as the argument
    .expect("Failed to read line");
  println!("You guessed: {}", guess);

}
