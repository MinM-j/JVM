use std::collections::VecDeque;
use libffi::middle::Type;

#[derive(Debug)]
pub struct ParsedDescriptor {
    pub arg_types: Vec<String>,  // List of argument types (e.g., ["Ljava/lang/String;", "[Ljava/lang/Object;"])
    pub return_type: String,     // Return type (e.g., "V", "I")
}

pub fn parse_descriptor(descriptor: &str) -> Result<ParsedDescriptor, String> {
    let mut chars: VecDeque<char> = descriptor.chars().collect();
    let mut arg_types = Vec::new();
    let mut current_arg = String::new();
    let mut in_args = false;

    // Check opening parenthesis
    if let Some('(') = chars.pop_front() {
        in_args = true;
    } else {
        return Err("Invalid descriptor: expected '('".to_string());
    }

    // Parse arguments
    while let Some(c) = chars.pop_front() {
        if in_args {
            match c {
                ')' => {
                    if !current_arg.is_empty() {
                        arg_types.push(current_arg);
                        current_arg = String::new();
                    }
                    in_args = false;
                }
                'L' => {
                    current_arg.push(c);
                    while let Some(next) = chars.pop_front() {
                        current_arg.push(next);
                        if next == ';' {
                            arg_types.push(current_arg);
                            current_arg = String::new();
                            break;
                        }
                    }
                }
                '[' => {
                    current_arg.push(c);
                    // Handle nested arrays (e.g., [[I)
                    while let Some(next) = chars.front() {
                        if *next == '[' {
                            current_arg.push(chars.pop_front().unwrap());
                        } else {
                            break;
                        }
                    }
                    if let Some(next) = chars.pop_front() {
                        current_arg.push(next);
                        if next == 'L' {
                            while let Some(nested) = chars.pop_front() {
                                current_arg.push(nested);
                                if nested == ';' {
                                    break;
                                }
                            }
                        }
                        arg_types.push(current_arg);
                        current_arg = String::new();
                    } else {
                        return Err("Invalid descriptor: incomplete array type".to_string());
                    }
                }
                'I' | 'J' | 'F' | 'D' | 'B' | 'S' | 'C' | 'Z' => {
                    current_arg.push(c);
                    arg_types.push(current_arg);
                    current_arg = String::new();
                }
                _ => return Err(format!("Unexpected character in descriptor: {}", c)),
            }
        } else {
            // After ')', parse return type
            if chars.is_empty() {
                return Ok(ParsedDescriptor {
                    arg_types,
                    return_type: c.to_string(),
                });
            } else {
                return Err("Invalid descriptor: extra characters after return type".to_string());
            }
        }
    }

    Err("Invalid descriptor: missing ')' or return type".to_string())
}

// Helper for parse_return_type (assuming it exists)
pub fn parse_return_type(return_type: &str) -> Result<Type, String> {
    match return_type {
        "V" => Ok(Type::void()),
        "I" | "B" | "S" | "C" | "Z" => Ok(Type::i32()),
        "J" => Ok(Type::i64()),
        "F" => Ok(Type::f32()),
        "D" => Ok(Type::f64()),
        "L" | "[" => Ok(Type::pointer()), // Simplified for references/arrays
        _ => Err(format!("Unsupported return type: {}", return_type)),
    }
}
