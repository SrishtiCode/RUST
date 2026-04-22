// Importing the Ordering enum from the standard comparison library
// Ordering has three variants: Less, Greater, and Equal
// Used to compare two values and determine their relationship
use std::cmp::Ordering;

// Importing the standard input/output library
// Needed for reading user input from the terminal
use std::io;

// Importing the Rng (Random Number Generator) trait from the rand crate
// The `rand` crate is an external dependency added in Cargo.toml
// Rng trait defines methods that random number generators implement
use rand::Rng;

fn main() {
    // Print the game title to the console
    println!("Guess the number!");

    // Generate a random secret number between 1 and 100 (inclusive)
    // thread_rng() returns a generator local to the current thread, seeded by the OS
    // gen_range(1..=100) uses an inclusive range — both 1 and 100 are possible outputs
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // Begin an infinite loop — the game keeps asking for guesses until the user wins
    loop {
        println!("Please input your guess.");

        // Declare a mutable variable `guess` to store user input
        // Variables in Rust are immutable by default; `mut` makes it mutable
        // String::new() creates a new, empty, growable UTF-8 string on the heap
        let mut guess = String::new();

        // Access standard input (stdin) to read from the terminal
        io::stdin()
            // read_line appends the user's input (including '\n') into `guess`
            // &mut guess passes a mutable reference — allows read_line to modify `guess`
            // without taking ownership of it
            .read_line(&mut guess)
            // read_line returns a Result<usize, Error>
            // .expect() unwraps the Result: panics with this message if an error occurs
            // Otherwise, it returns the number of bytes read (which we ignore here)
            .expect("Failed to read line");

        // Shadow the previous `guess` (String) with a new `guess` (u32)
        // Shadowing lets us reuse the same variable name for a different type
        // .trim() removes leading/trailing whitespace and the trailing newline '\n'
        // .parse() attempts to convert the trimmed string into a u32
        // Returns a Result<u32, ParseIntError>
        let guess: u32 = match guess.trim().parse() {
            // If parsing succeeds, bind the parsed number to `num` and return it
            Ok(num) => num,
            // If parsing fails (e.g., user typed letters), ignore the error and
            // restart the loop — asking the user to try again
            Err(_) => continue,
        };

        // Display the validated guess back to the user
        // {} is a placeholder that uses the Display trait to print the value
        println!("You guessed: {guess}");

        // Compare the user's guess to the secret number using the cmp() method
        // cmp() returns an Ordering variant: Less, Greater, or Equal
        match guess.cmp(&secret_number) {
            // Guess is lower than the secret number
            Ordering::Less => println!("Too small!"),

            // Guess is higher than the secret number
            Ordering::Greater => println!("Too big!"),

            // Guess matches the secret number — player wins!
            Ordering::Equal => {
                println!("You win!");
                // Exit the loop (and the program) since the correct number was found
                break;
            }
        }
    } // End of loop
} // End of main

// ============================================================
// BACKGROUND NOTES
// ============================================================
//
// PRELUDE:
//   Rust automatically imports a set of commonly used items into every
//   program's scope. This is called the "prelude".
//   For anything NOT in the prelude (like `io` or `rand`), we explicitly
//   bring it into scope using `use`.
//
// REFERENCES (&):
//   The `&` symbol creates a reference — it lets multiple parts of code
//   access the same memory without copying it.
//   References are immutable by default; `&mut` makes them mutable.
//
// RESULT TYPE:
//   Many Rust functions return `Result<T, E>` — an enum with two variants:
//     Ok(T)  — the operation succeeded, contains the value
//     Err(E) — the operation failed, contains the error
//   This forces explicit error handling, making Rust programs more robust.
//
// SHADOWING:
//   Rust allows re-declaring a variable with the same name using `let`.
//   The new variable "shadows" the old one. Unlike `mut`, shadowing can
//   change the type of the variable (String → u32 here).


/*OUTPUT
cargo run
   Compiling guessing_game v0.1.0 (/home/srishti/Desktop/Rust/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.17s
     Running `target/debug/guessing_game`
Guess the number !
Please input your guess.
35
You guessed: 35
Too small!
Please input your guess.
60
You guessed: 60
Too small!
Please input your guess.
80
You guessed: 80
Too big!
Please input your guess.
70
You guessed: 70
Too big!
Please input your guess.
65
You guessed: 65
Too small!
Please input your guess.
67
You guessed: 67
Too small!
Please input your guess.
68
You guessed: 68
Too small!
Please input your guess.
69
You guessed: 69
You win!

*/
