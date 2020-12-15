use rand::prelude::*;
use rand::rngs::OsRng;

pub fn random_element_rng<'a, T>(vec: &'a mut Vec<T>, rng: &'a mut dyn RngCore) -> &'a T {
    &vec[rng.gen_range(0, vec.len())]
}

#[allow(dead_code)]
pub fn random_element<'a, T>(vec: &'a mut Vec<T>) -> &'a T {
    &vec[OsRng.gen_range(0, vec.len())]
}