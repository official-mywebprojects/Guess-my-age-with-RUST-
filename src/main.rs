use std::io;
//use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess My Age"); 

    let my_age = 30;

    //let secret_number = rand::thread_rng().gen_range(1..=50);

    loop {
	println!("Enter a number");

//accepting user input
    let mut age = String::new();
    
//reading user input
    io::stdin().read_line(&mut age).expect("Failed to read line");

//checking and converting input to number from String type
    let age: u32 = match age.trim().parse(){
    Ok(num) => num,
    Err(_) => continue,
};

//printing user age
    println!("You Guessed: {age}");

//comparing age to secret number
    match age.cmp(&my_age) {
  Ordering::Less => println!("No...No, Too small 😏"),
  Ordering::Greater => println!("I'm not that old 😆 "),
  Ordering::Equal => {println!("YOU GOT IT !!😍😍"); break;}
}
}
}