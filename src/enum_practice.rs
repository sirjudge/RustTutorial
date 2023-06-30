// since we have both unused methods and unused
// variables as this is a tutorial we can ignore these warnings
#![allow(unused_variables)]
#![allow(dead_code)]

pub enum NavigationAids{
    NDB = 5,
    VOR,
    VORDME,
 //   FIX {name:String, latitude:f32, longitude:f32}
}

pub fn print_navigation_aid_enum(){
    println!("NDB:\t{}",NavigationAids::NDB as u8);
    println!("VOR:\t{}",NavigationAids::VOR as u8);
    println!("VORDME:\t{}",NavigationAids::VORDME as u8);
}

