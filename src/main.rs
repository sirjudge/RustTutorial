// since we have both unused methods and unused
// variables as this is a tutorial we can ignore these warnings
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_assignments)]
use crate::enum_practice::print_navigation_aid_enum;
use crate::option_demo::option_demo;

mod find_distance;
mod demo_code;
mod if_statement;
mod enum_practice;
mod option_demo;

fn main(){
    option_demo();
}
