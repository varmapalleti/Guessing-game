
use std::io;
use rand::prelude::IndexedRandom;
use rand::rng;



fn main() {
    println!("🎴 Welcome to the Card Guessing Game! 🎴");
    println!("Try to guess a card from the same set as the secret card!");
    println!("Amar = A,2,3,4,5,6");
    println!("Akbar  = 7,8,9,10");
    println!("Anthony = J,Q,K");


    let amar = ["A", "2", "3", "4", "5", "6"];
    let akbar = ["7", "8", "9", "10"];
    let anthony = ["J", "Q", "K"];

  
    let deck: Vec<&str> = amar.iter().chain(akbar.iter()).chain(anthony.iter()).copied().collect();

    loop {
    let mut rng = rng();
    let secret_card = deck.choose(&mut rng).unwrap(); 

    let secret_category = if amar.contains(secret_card) {
        "Amar"
    } else if akbar.contains(secret_card) {
        "Akbar"
    } else {
        "Anthony"
    };

        println!("\nEnter your card guess (A, 2-10, J, Q, K):");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");

        let guess = guess.trim(); 

        if !deck.contains(&guess) {
            println!("❌ Invalid card! Please guess a valid card from the deck.");
            continue;
        }

        let guess_category = if amar.contains(&guess) {
            "Amar"
        } else if akbar.contains(&guess) {
            "Akbar"
        } else {
            "Anthony"
        };

        println!("Your guess '{}' belongs to the '{}' division.", guess, guess_category);

        if guess_category == secret_category {
            println!("🎉 Congratulations! You guessed from the correct set ('{}')! The secret card was '{}' 🎉", secret_category, secret_card);
            break;
        } else {
            println!("❌ Wrong guess! The secret card '{}' belongs to '{}'. Try again!", secret_card, secret_category);
        }
    }
}
