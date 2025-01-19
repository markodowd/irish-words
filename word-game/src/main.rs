use rand::seq::SliceRandom; // For selecting a random element from a collection
use std::fs::File; // For handling file operations (opening files)
use std::io::{self, BufRead, Write}; // For reading input/output and buffered file reading
use std::path::Path; // For handling file paths

fn main() -> io::Result<()> {
    let stars = "*".repeat(20);

    // Define file paths relative to the 'guessing-game' folder
    let files = vec![
        "../a.txt", "../b.txt", "../c.txt", "../d.txt", "../e.txt", "../f.txt", "../g.txt",
        "../h.txt", "../i.txt", "../l.txt", "../m.txt", "../n.txt", "../o.txt", "../p.txt",
        "../r.txt", "../s.txt", "../t.txt", "../u.txt", "../v.txt",
    ];

    // Read all words from the specified files into a vector
    let words = read_words_from_files(files)?;
    if words.is_empty() {
        // If no words were found, print an error message and exit the program
        eprintln!("No words found in the provided files.");
        return Ok(()); // Graceful exit
    }

    // Welcome message
    println!("Welcome to the typing game! Type 'quit' to exit.");
    println!("{}", stars);

    // Create a random number generator
    let mut rng = rand::thread_rng();

    // Infinite game loop
    loop {
        // Pick a random word from the list
        let chosen_word = words.choose(&mut rng).expect("Failed to choose a word.");

        // Prompt the user to type the randomly selected word
        println!("Type the word: {}", chosen_word);
        print!("Your input: "); // Prompt displayed for user input
        io::stdout().flush()?; // Flush the output to ensure the prompt is shown immediately

        // Read the user's input
        let mut input = String::new(); // A buffer to store the user input
        io::stdin().read_line(&mut input)?; // Read a line from standard input
        let trimmed_input = input.trim(); // Remove any leading/trailing whitespace

        // Check if user wants to quit the game
        if trimmed_input.eq_ignore_ascii_case("quit") {
            // If the user types "quit" (case-insensitive), exit the game
            println!("Thanks for playing");
            break; // Exit the loop and end the program
        }

        if trimmed_input == *chosen_word {
            // If the input matches the word, print "Correct!"
            println!("Correct!\n");
        } else {
            // If the input does not match, show the correct word
            println!("Incorrect. The correct word was: {}\n", chosen_word);
        }

        println!("{}", stars);
    }

    Ok(()) // Return success
}

// Function to read words from a list of files
fn read_words_from_files(files: Vec<&str>) -> io::Result<Vec<String>> {
    let mut words = Vec::new(); // Initalise an empty vector to store words

    for file in files {
        let path = Path::new(file); // Convert the file path string into a 'Path' object
        if !path.exists() {
            // Check if the file exists before trying to open it
            eprintln!("Warning: File not found: {:?}", path); // Print a warning for missing files
            continue; // Skip to the next file
        }

        let file = File::open(path)?; // Open the file for reading
        let reader = io::BufReader::new(file); // Create a buffered reader for efficient line reading

        for line in reader.lines() {
            // Iterate through each line in the file
            if let Ok(word) = line {
                // Check if the line was read successfully
                let trimmed_word = word.trim().to_string(); // Remove leading/trailing whitespace
                if !trimmed_word.is_empty() {
                    // Ignore empty lines
                    words.push(trimmed_word); // Add the cleaned word to the vector
                }
            }
        }
    }

    Ok(words) // Return the list of words
}
