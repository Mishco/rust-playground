use rand::Rng;
use std::cmp::Ordering;
use std::io;


pub fn guessing_game() {
    println!("Guess the number!");
    
    let secret_number = rand::thread_rng().gen_range(1..=100);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, y, _z) = tup;

    println!("The value of y is: {y}");
    
    println!("The secret number is: {secret_number}");

    loop { 
        println!("Please input your guess.");
    
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }

    // let guess: u32 = "42".parse().expect("Not a number");

    // let x = 5;
    // let y = 10;

    // println!("x = {x} and y +2 = {}", y + 2);
}

#[test]
fn test_guessing_game() {

    assert_eq!(true, true);
}
