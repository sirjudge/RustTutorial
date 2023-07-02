pub fn for_loop(){
    let mut counter = 0;
    loop{
        println!("{}",counter);
        counter += 1;
        if counter == 5{
            continue;
        }
       if counter == 10{
            break;
        }

    }
}

pub fn while_loop(){
    let mut counter = 1;
    while counter <= 10{
        println!("{}",counter);
        counter += 1;
    }
}

pub fn range_loop(){
    for index in 1..11{
        println!("{}",index);
    }
}

pub fn rang_loop_inclusive(){
    for index in 1..=10{
        println!("{}",index);
    }
}

pub fn rust_iterator(){
    let duck_aircraft = ["boeing 737", "boeing 767", "boeing 787","airbus 319","Airbus 320"];

    for aircraft in duck_aircraft.iter() {
        println!("{}",aircraft);
    }
}