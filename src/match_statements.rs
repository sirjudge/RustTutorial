pub fn match_demo(){
    let animal = "Duck";
    match animal {
        "Duck" => println!("Quack"),
        "Dog" => println!("Bark!"),
        _ => println!("None")
    }
}


pub fn valid_frequency(){
    let ndb_freq:u16 = 300;
    match ndb_freq {
        100..=500 => {
            println!("NDB frequency valid");
        }
        _ => {
            println!("NDB frequency is not valid");
        }
    }
}

pub fn match_with_operators(){
    let ndb_freq:u16 = 384;

    match ndb_freq{
        ndb_freq if ndb_freq >= 200 && ndb_freq <= 500 => {
            println!("NDB Frequency is valid");
        }
        _ => {
            println!("Not valid");
        }
    }
}