//Aufgabe 1
pub fn say_hello() -> String {
    "Hello, tests!".to_string()
}



// Aufgabe 2
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

//Aufgabe 3
pub fn parse_number(input: &str) -> Result<u32, String>  {
    match input.trim().parse::<u32>() {
        Ok(n) => Ok(n),
        Err(_) => Err(format!("'{}' ist keine gültige Zahl!", input))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //Aufgabe 1
    #[test]
    fn it_says_hello() {
        assert_eq!(say_hello(), "Hello, tests!");
    }

    //Aufgabe 2
    #[test]
    fn greets_by_name(){
        assert_eq!(greet("Paul"), "Hello, Paul!");
        assert_eq!(greet("Rust"), "Hello, Rust!");
        assert_eq!(greet("🦀"), "Hello, 🦀!");
    }

    //Aufgabe 3
    #[test]
    fn parses_valid_number() {
        let result = parse_number("42");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
}

#[test]
    fn fails_on_invalid_input() {
        let result = parse_number("abc");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "'abc' ist keine gültige Zahl");
}


}