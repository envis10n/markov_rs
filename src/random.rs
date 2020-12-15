use rand::prelude::*;
use rand::rngs::OsRng;

pub fn random_element_rng<'a, T>(vec: &'a mut Vec<T>, rng: &'a mut dyn RngCore) -> &'a T {
  &vec[rng.gen_range(0, vec.len())]
}

#[allow(dead_code)]
pub fn random_element<'a, T>(vec: &'a mut Vec<T>) -> &'a T {
  &vec[OsRng.gen_range(0, vec.len())]
}

#[cfg(test)]
mod random_tests {
  #[test]
  fn rand_element() {
    let mut a: Vec<u8> = vec![0xff, 0x20, 0x30];
    let b: u8 = *super::random_element(&mut a);
    let res: bool = b == 0xff || b == 0x20 || b == 0x30;
    assert!(res);
  }
}
