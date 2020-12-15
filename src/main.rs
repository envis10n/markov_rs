use rand::prelude::*;
use crate::markov::MarkovGenerator;

mod markov;
mod random;

fn main() {
    let mut rng = StdRng::from_entropy();
    let mut mk: MarkovGenerator = MarkovGenerator::new(&mut rng);
    mk.parse("Hello, world!\nEat shit and die!\nPeople suck so much, I hate them.", 1);
    println!("{}", mk.generate(100));
}
