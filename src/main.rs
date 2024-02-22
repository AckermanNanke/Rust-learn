use crate::generics::run_generics_demo;
use crate::io_demo::run_io_demo1;
use crate::lifetime_demo::run_lifetime_demo;
use crate::trait_demo::run_trait_demo;
mod generics;
mod io_demo;
mod lifetime_demo;
mod trait_demo;

fn main() {
    // run_learn_demo();
    // run_generics_demo();
    // run_trait_demo();
    // run_lifetime_demo();
    run_io_demo1();
}
