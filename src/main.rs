use crate::generics::runGenericsDemo;
use crate::learnmod::runLearnDemo;
use crate::lifetime_demo::run_lifetime_demo;
use crate::trait_demo::run_trait_demo;
mod generics;
mod learnmod;
mod lifetime_demo;
mod trait_demo;

fn main() {
    // runLearnDemo();
    // runGenericsDemo();
    // run_trait_demo();
    run_lifetime_demo();
}
