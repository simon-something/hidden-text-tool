use std::io::{self, Write};
use std::env;

const LANGUAGE_TAG_START: char = '\u{E0001}';
const CANCEL_TAG: char = '\u{E007F}';
const FULL_STOP_TAG: char = '\u{E002E}';
const TAG_BASE: u32 = 0xE0000;

struct HiddenTextTool {
    // We'll implement the encoding and decoding logic here
}

impl HiddenTextTool {
    fn new() -> Self {
        Self {}
    }

    fn encode_text(&self, input: &str) -> String {
        let mut result = String::new();
        
        // Start with language tag
        result.push(LANGUAGE_TAG_START);
        
        // Convert each character to its hidden Unicode equivalent
        for ch in input.chars() {
            let code_point = ch as u32;
            let hidden_char = char::from_u32(TAG_BASE + code_point)
                .unwrap_or('\u{E003F}'); // Use '?' tag as fallback
            result.push(hidden_char);
        }
        
        // End with full stop and cancel
        result.push(FULL_STOP_TAG);
        result.push(CANCEL_TAG);
        
        result
    }
    
    fn decode_text(&self, input: &str) -> Result<String, String> {
        let chars: Vec<char> = input.chars().collect();
        
        if chars.is_empty() {
            return Err("Empty input".to_string());
        }
        
        // Check if it starts with language tag
        if chars[0] != LANGUAGE_TAG_START {
            return Err("Text doesn't start with language tag".to_string());
        }
        
        // Find the end markers
        let mut end_pos = None;
        for (i, &ch) in chars.iter().enumerate().rev() {
            if ch == CANCEL_TAG && i > 0 && chars[i-1] == FULL_STOP_TAG {
                end_pos = Some(i - 1);
                break;
            }
        }
        
        let end_pos = end_pos.ok_or("Invalid format: missing end markers")?;
        
        // Decode the content between markers
        let mut result = String::new();
        for &ch in &chars[1..end_pos] {
            let code_point = ch as u32;
            if code_point >= TAG_BASE {
                let original_code = code_point - TAG_BASE;
                if let Some(original_char) = char::from_u32(original_code) {
                    result.push(original_char);
                } else {
                    result.push('?'); // Fallback for invalid codes
                }
            } else {
                return Err(format!("Invalid hidden character found: {}", ch));
            }
        }
        
        Ok(result)
    }
    
    fn run_interactive(&self) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            println!("\n=== Hidden Text Encoder/Decoder ===");
            println!("1. Encode text (convert to hidden characters)");
            println!("2. Decode text (reveal hidden message)");
            println!("3. Exit");
            print!("\nChoose an option (1-3): ");
            io::stdout().flush()?;
            
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let choice = input.trim();
            
            match choice {
                "1" => self.handle_encode()?,
                "2" => self.handle_decode()?,
                "3" => {
                    println!("Goodbye!");
                    break;
                }
                _ => println!("Invalid choice. Please enter 1, 2, or 3."),
            }
        }
        Ok(())
    }
    
    fn handle_encode(&self) -> Result<(), Box<dyn std::error::Error>> {
        print!("\nEnter text to encode: ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let text = input.trim();
        
        if text.is_empty() {
            println!("No text entered.");
            return Ok(());
        }
        
        let encoded = self.encode_text(text);
        println!("\nEncoded text (copy the content including quotes):");
        println!("\"{}\"", encoded);
        
        Ok(())
    }
    
    fn handle_decode(&self) -> Result<(), Box<dyn std::error::Error>> {
        print!("\nEnter hidden text to decode: ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let text = input.trim();
        
        // Remove surrounding quotes if present
        let text = if (text.starts_with('"') && text.ends_with('"')) ||
                     (text.starts_with('\'') && text.ends_with('\'')) {
            &text[1..text.len()-1]
        } else {
            text
        };
        
        if text.is_empty() {
            println!("No text entered.");
            return Ok(());
        }
        
        match self.decode_text(text) {
            Ok(decoded) => {
                // Print in red using ANSI escape codes
                println!("\nDecoded message: \x1b[31m{}\x1b[0m", decoded);
            }
            Err(e) => {
                println!("Error decoding: {}", e);
            }
        }
        
        Ok(())
    }
}

fn print_usage() {
    println!("Hidden Text Tool v1.0");
    println!("Encode and decode text using hidden Unicode characters\n");
    println!("Usage:");
    println!("  hidden_text_tool                    # Interactive mode");
    println!("  hidden_text_tool -e \"text\"          # Encode text");
    println!("  hidden_text_tool --encode \"text\"    # Encode text");
    println!("  hidden_text_tool -d \"hidden\"       # Decode hidden text"); 
    println!("  hidden_text_tool --decode \"hidden\" # Decode hidden text");
    println!("  hidden_text_tool -h                 # Show this help");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let tool = HiddenTextTool::new();
    
    match args.len() {
        1 => {
            // No arguments - run in interactive mode
            if let Err(e) = tool.run_interactive() {
                eprintln!("Error: {}", e);
            }
        }
        2 => {
            if args[1] == "-h" || args[1] == "--help" {
                print_usage();
            } else {
                println!("Error: Invalid arguments. Use -h for help.");
            }
        }
        3 => {
            match args[1].as_str() {
                "-e" | "--encode" => {
                    let encoded = tool.encode_text(&args[2]);
                    println!("\"{}\"", encoded);
                }
                "-d" | "--decode" => {
                    // Remove quotes if present
                    let text = if (args[2].starts_with('"') && args[2].ends_with('"')) ||
                                  (args[2].starts_with('\'') && args[2].ends_with('\'')) {
                        &args[2][1..args[2].len()-1]
                    } else {
                        &args[2]
                    };
                    
                    match tool.decode_text(text) {
                        Ok(decoded) => {
                            // Print in red using ANSI escape codes
                            println!("Decoded: \x1b[31m{}\x1b[0m", decoded);
                        }
                        Err(e) => {
                            eprintln!("Error: {}", e);
                        }
                    }
                }
                _ => {
                    println!("Error: Invalid option '{}'. Use -h for help.", args[1]);
                }
            }
        }
        _ => {
            println!("Error: Too many arguments. Use -h for help.");
        }
    }
}
