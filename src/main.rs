use crate::generics::runGenericsDemo;
use crate::learnmod::runLearnDemo;
use crate::traitDemo::run_trait_demo;
mod generics;
mod learnmod;
mod traitDemo;

fn main() {
    // runLearnDemo();
    // runGenericsDemo();
    run_trait_demo();
}
