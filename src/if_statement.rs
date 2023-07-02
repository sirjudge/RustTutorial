// since we have both unused methods and unused
// variables as this is a tutorial we can ignore these warnings
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(irrefutable_let_patterns)]

pub fn if_statement_example() {
    let word = "Duck";
    if word == "Duck" {
        println!("quack!");
    } else if word == "dog" {
        println!("bark");
    } else {
        println!("neither");
    }
}

pub fn airplane_if(){
    let available_aircraft = "boeing";
    let minimum_crew = 7;
    let available_crew = 4;


    if available_aircraft == "boeing" || available_aircraft == "Airbus" && minimum_crew < available_crew{
        println!("okay");
    }
}

pub fn if_let_statement(){
    let animal = "duck";
    if let animal = "duck" {
        println!("quack!");
    }

}