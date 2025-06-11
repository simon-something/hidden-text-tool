# Hidden Text Tool

A Rust tool for encoding and decoding text using hidden Unicode characters (Tag block U+E0000 to U+E007F).

## Features

- **Encode text**: Convert regular text to invisible Unicode characters
- **Decode text**: Reveal hidden messages in text that appears to contain only invisible characters
- **Interactive mode**: User-friendly command-line interface
- **Command-line arguments**: Direct encoding/decoding from command line
- **Visual feedback**: Decoded text is displayed in red for easy identification

## How it works

The tool uses Unicode Tag characters (U+E0000 to U+E007F) to hide text:

1. **Encoding**: 
   - Starts with Language Tag (U+E0001)
   - Each character is converted to its Tag equivalent (original codepoint + 0xE0000)
   - Ends with Full Stop Tag (U+E002E) + Cancel Tag (U+E007F)
   - Output is surrounded by quotes for easy copying

2. **Decoding**:
   - Validates the format (proper start/end tags)
   - Converts Tag characters back to regular Unicode
   - Displays result in red for visibility

## Usage

### Interactive Mode
```bash
./target/release/hidden_text_tool
```

### Command Line
```bash
# Encode text
./target/release/hidden_text_tool -e "Hello World!"
./target/release/hidden_text_tool --encode "Your secret message"

# Decode hidden text
./target/release/hidden_text_tool -d "encoded_string_here"
./target/release/hidden_text_tool --decode "encoded_string_here"

# Show help
./target/release/hidden_text_tool -h
```

## Building

```bash
cd hidden_text_tool
cargo build --release
# The binary will be at ./target/release/hidden_text_tool
```

## Testing

Run the Rust tests:
```bash
cargo test
```