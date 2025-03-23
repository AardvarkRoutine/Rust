//Aufgabe 1
pub fn say_hello() -> String {
    "Hello, tests!".to_string()
}



// Aufgabe 2
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
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

}