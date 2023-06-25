fn main() {
    //tuples();
    //creating_strings();
    //string_concatination();
    variable_types();
}

fn tuples() {
    let location = ("KCLE", 41.4094069,-81.8546911);
    let (name,latitude,longitude) = location;
    println!("locationName:{name}, latitude:{latitude}, longitude:{longitude}");
}

fn creating_strings(){
    let person_name_string = String::from("Nico judge");
    let person_name_slice = &person_name_string;
    let person_name_slice2 = person_name_slice.as_str();
}

fn string_concatination(){
    let duck = "Duck";
    let airlines = "Airlines";
    let airline_name = format!("{} {}",duck,airlines);
    println!("{}",airline_name);

    let mut slogan = String::new();
    slogan.push_str("we hit the ground");
    slogan.push(' ');
    slogan = slogan + "!";
    println!("{}",slogan);
}

fn variable_types(){
    let my_variable_name: u32 = 0;
    
}