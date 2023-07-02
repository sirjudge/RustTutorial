pub fn no_value(){
    let test_string = String::from("Duck Airlines");
    let letter = test_string.chars().nth(16);

    let value = match letter {
        Some(character) => character.to_string(),
        None => String::from("No Value")
    };

    println!("{}",value);
}