// since we have both unused methods and unused
// variables as this is a tutorial we can ignore these warnings
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_assignments)]
use crate::enum_practice::{navigation_aid_enum, NavigationAids, print_nav_aid};
use crate::find_distance::main_run;
use crate::if_statement::if_let_statement;
use crate::loop_fun::{for_loop, range_loop, rust_iterator, while_loop};
use crate::match_statements::{match_demo, match_with_operators, valid_frequency};

mod find_distance;
mod demo_code;
mod if_statement;
mod enum_practice;
mod option_demo;
mod match_statements;
mod loop_fun;

fn main(){
    main_run();
}
