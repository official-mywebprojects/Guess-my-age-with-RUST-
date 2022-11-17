use std::io;
//use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("\n\nGUESS MY AGE AND SET YOUR AGE: GAME\n===================================\nGuess My Age"); 
    //set your new age
    let my_age = 30;

    // let secret_number = rand::thread_rng().gen_range(1..=50);

     loop {
        println!("\nEnter a number");
    //awaiting user input
        let mut age = String::new();
        
    //reading user input
        io::stdin().read_line(&mut age).expect("Failed");

    //checking and converting input to number from String type
        let age: u32 = match age.trim().parse(){
        Ok(num) => num,
        Err(_) => continue,
    };

    //printing age
        println!("\nYour Guess is: {age}");

        //comparing age to my_age
         match age.cmp(&my_age) {
          Ordering::Less => println!("\nNo...No, Too small"),
          Ordering::Greater => println!("\nI'm not that old"),
          Ordering::Equal => {println!("\nCorrect!!"); your_turn(); break;}
        } 
    }
    
    

    fn your_turn(){
        loop{
            println!("\n\n========= Now it's your turn. =========\nSet your age and let someone guess it!\n======================================\nI AM: ");

            let mut new_age = String::new();

            io::stdin().read_line(&mut new_age).expect("Failed");

            let new_age: u32 = match new_age.trim().parse(){
                Ok(num) => num,
                Err(_) => continue,
            }; 

            let my_age = new_age;
            
            start_game(my_age); break;
        }
    }


    fn start_game(xx: u32) {
        println!("\n======= ALL SET =======\nYour New Age Is: {}", xx);
        println!("\n\nNow set 'my_age' on line 8 to your new age and let others guess it!");
        end_game();
    }

    fn end_game(){
        println!("\nThanks for playing !");
    }

}