
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

fn variable_types_and_casting(){
    let unsigned_int: u32 = 0;
    let auto_signed_int = 0;

    let float_thirty_two: f32 = 17.2;
    let unsigned_eight: u8 = 5;
    let cast_unsigned_eight = unsigned_eight as f32;
    let result = float_thirty_two / cast_unsigned_eight;
    println!("{}",result);
}

fn variable_immutability(){
    let mut changeable_variable = 500;
    let mutable_variable = 500;
    changeable_variable = 600;
}

fn scopes(){
    let scope_test = "outerscope";
    println!("{}", scope_test);
    {
        let scope_test = "inner scope";
        println!("{}",scope_test);
    }
    println!("{}", scope_test);
}

fn operators(){
    let modulus = 18 % 7;
    println!("{}",modulus);

    let squared = i32::pow(8,2);
    let float_integer = f32::powi(6.5,3);
    let float_float = f32::powf(6.5,3.14);
    println!("{}",squared);
    println!("{}",float_integer);
    println!("{}",float_float);
}

fn boolean_types(){
    let are_equal_is_true = 1 == 1;
    let are_equal_is_false = 1 == 2;
    let are_not_equal = 1 != 2;

    let is_true = true;
    let is_false = !is_true;
    println!("is_true:{} is_false:{}", is_true,is_false );
}

fn logic_operators(){
    let have_driver_license = false;
    let have_passport = true;
    let have_proof = have_passport || have_driver_license;
    let have_boarding_pass = true;
    let have_id = have_proof;
    let can_board = have_boarding_pass && have_id;
    println!("boarding Pass:{}, ID:{}",have_boarding_pass,have_id);
    println!("can baord plane:{}",can_board);

    let first = 10;
    let second = 15;
    let results = first < second;
    println!("{}",results);
}

fn bitwise_operators(){
    let bitwise_and = 86 & 27;
    println!("bitwise and:{}", bitwise_and);

    let bitwise_or = 86 | 27;
    println!("bitwise_or and:{}", bitwise_or);

    let bitwise_xor = 86 ^ 27;
    println!("bitwise_xor and:{}", bitwise_xor);

    let left_shift = 86 << 1;
    println!("left_shift and:{}", left_shift);

    let right_shift = 86 >> 1;
    println!("right_shift and:{}", right_shift);
}