use std::collections::HashMap;
use std::fs;
use clap::Parser;

// Define command-line arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The text to convert, or path to a file to read from
    #[arg(short, long)]
    text: Option<String>,

    /// Convert from American to British English
    #[arg(short = 'b', long, default_value_t = false)]
    to_british: bool,

    /// Convert from British to American English
    #[arg(short = 'a', long, default_value_t = false)]
    to_american: bool,

    /// Path to American spellings JSON file
    #[arg(long, default_value = "american_spellings.json")]
    american_path: String,

    /// Path to British spellings JSON file
    #[arg(long, default_value = "british_spellings.json")]
    british_path: String,
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let text_to_convert: String;
    if let Some(input_text) = args.text {
        // Check if it's a file path or direct text
        if fs::metadata(&input_text).is_ok() { // Check if it's a valid path
            text_to_convert = fs::read_to_string(&input_text)?;
        } else {
            text_to_convert = input_text;
        }
    } else {
        println!("No text provided. Please provide text with -t or --text, or pipe input.");
        println!("Example: cargo run -- -t \"color\" --to-british");
        return Ok(());
    }

    // Load data based on specified paths
    let american_to_british_raw = fs::read_to_string(&args.american_path)?;
    let american_to_british_map: HashMap<String, String> = serde_json::from_str(&american_to_british_raw)?;

    let british_to_american_raw = fs::read_to_string(&args.british_path)?;
    let british_to_american_map: HashMap<String, String> = serde_json::from_str(&british_to_american_raw)?;

    let converted_text: String;
    if args.to_british && args.to_american {
        eprintln!("Error: Cannot convert to both British and American English simultaneously.");
        std::process::exit(1);
    } else if args.to_british {
        converted_text = convert_text(&text_to_convert, &american_to_british_map);
    } else if args.to_american {
        converted_text = convert_text(&text_to_convert, &british_to_american_map);
    } else {
        println!("No conversion direction specified. Use --to-british or --to-american.");
        converted_text = text_to_convert; // No conversion, just echo
    }

    println!("{}", converted_text);

    Ok(())
}


fn apply_case_to_replacement(original_word: &str, replacement: &str) -> String {
    if original_word.chars().next().map_or(false, |c| c.is_uppercase()) {
        // If original starts with uppercase, make replacement start with uppercase
        let mut chars = replacement.chars();
        if let Some(first_char) = chars.next() {
            return first_char.to_uppercase().chain(chars).collect();
        }
    } else if original_word.chars().all(|x| x.is_uppercase()) {
        // If original is ALL CAPS, make replacement ALL CAPS
        return replacement.to_uppercase();
    }
    // Otherwise, return replacement as is (assuming original was lowercase or mixed)
    replacement.to_string()
}


fn convert_text(text: &str, mapping: &HashMap<String, String>) -> String {
    let mut converted_text = String::new();
    let mut current_word = String::new();

    for c in text.chars() {
        if c.is_alphabetic() {
            current_word.push(c);
        } else {
            // We've hit a non-alphabetic character, so the previous sequence was a word (or part of one)
            if !current_word.is_empty() {
                // Try to convert the word
                let lowercased_word = current_word.to_lowercase(); // For case-insensitive lookup
                if let Some(replacement) = mapping.get(&lowercased_word) {
                    // If a replacement is found, use it.
                    converted_text.push_str(&apply_case_to_replacement(&current_word, replacement));
                } else {
                    // No replacement found, append the original word
                    converted_text.push_str(&current_word);
                }
                current_word.clear(); // Reset for the next word
            }
            converted_text.push(c); // Add the non-alphabetic character
        }
    }

    // Handle the last word if the text ends with one
    if !current_word.is_empty() {
        let lowercased_word = current_word.to_lowercase();
        if let Some(replacement) = mapping.get(&lowercased_word) {
            converted_text.push_str(&apply_case_to_replacement(&current_word, replacement));
        } else {
            converted_text.push_str(&current_word);
        }
    }

    converted_text
}
