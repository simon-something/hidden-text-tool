use std::process::Command;

#[test]
fn test_encode_decode_functionality() {
    let test_cases = vec![
        "Hello World!",
        "This is a secret message",
        "Special chars: !@#$%^&*()",
        "Numbers: 0123456789",
    ];

    for test_text in test_cases {
        // Test encoding
        let encode_output = Command::new("./target/release/hidden_text_tool")
            .args(["-e", test_text])
            .output()
            .expect("Failed to execute encode command");
        
        assert!(encode_output.status.success(), "Encoding failed for: {}", test_text);
        
        let encoded_string = String::from_utf8(encode_output.stdout)
            .expect("Invalid UTF-8 in encode output");
        
        // Remove quotes and trim
        let encoded = encoded_string.trim()
            .strip_prefix('"')
            .and_then(|s| s.strip_suffix('"'))
            .expect("Encoded output should be quoted");
        
        // Test decoding
        let decode_output = Command::new("./target/release/hidden_text_tool")
            .args(["-d", &format!("\"{}\"", encoded)])
            .output()
            .expect("Failed to execute decode command");
        
        assert!(decode_output.status.success(), "Decoding failed for: {}", test_text);
        
        let decoded_output = String::from_utf8(decode_output.stdout)
            .expect("Invalid UTF-8 in decode output");
        
        // The output should contain the original text (removing ANSI color codes)
        let cleaned_output = decoded_output
            .replace("\x1b[31m", "")
            .replace("\x1b[0m", "")
            .replace("Decoded: ", "");
        
        let decoded_text = cleaned_output.trim();
        
        assert_eq!(decoded_text, test_text, "Round-trip failed for: {}", test_text);
    }
}
