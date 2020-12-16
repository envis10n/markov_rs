use crate::random::*;
use rand::prelude::*;
use std::cmp::min;
use std::collections::HashMap;
use rand::rngs::OsRng;

mod random;

pub struct MarkovGenerator<'a> {
    rng: Option<&'a mut dyn RngCore>,
    prefix: Vec<String>,
    suffix: HashMap<String, Vec<String>>,
}

fn get_word_count(v: &[String]) -> usize {
    let v2: String = v.join(" ");
    let v3: Vec<&str> = v2.split(" ").collect();
    v3.len()
}

impl<'a> MarkovGenerator<'a> {
    pub fn with_rng(rng: &'a mut dyn RngCore) -> Self {
        MarkovGenerator {
            rng: Some(rng),
            prefix: vec![],
            suffix: HashMap::new(),
        }
    }
    pub fn new() -> Self {
        MarkovGenerator {
            rng: None,
            prefix: vec![],
            suffix: HashMap::new(),
        }
    }
    pub fn generate(&mut self, length: i32) -> String {
        let mut res: Vec<String> = vec![];
        let mut osrng = OsRng.clone();
        let mut rng: &mut dyn RngCore = &mut osrng;
        if let Some(r) = &mut self.rng {
            rng = *r;
        }
        let mut prefix: String = random_element_rng(&mut self.prefix, rng).clone();
        let mut suffix: String;
        res.push(prefix.clone());
        while get_word_count(&res) < length as usize {
            match self.suffix.get_mut(&prefix) {
                Some(suf) => {
                    if suf.len() > 1 {
                        suffix = random_element_rng(suf, rng).clone();
                    } else {
                        suffix = suf[0].clone();
                    }
                    res.push(suffix.clone());
                    let t: Vec<&str> = prefix.split(" ").collect();
                    prefix = String::from(*t.last().unwrap()) + " " + &suffix;
                }
                None => {
                    prefix = random_element_rng(&mut self.prefix, rng).clone();
                }
            }
        }
        res.join(" ")
    }
    pub fn parse(&mut self, input: &str, n: usize) {
        let temp: Vec<&str> = input.split(" ").collect();
        for i in 0..temp.len() {
            let ix: usize = min(temp.len() - 1, i + n);
            let t = &temp[i..ix];
            let prefix: &str = &t.join(" ");
            if prefix.len() > 0 {
                let suffix: &str = temp[ix];
                match self.suffix.get_mut(prefix) {
                    Some(suf) => {
                        suf.push(String::from(suffix));
                    }
                    None => {
                        self.suffix
                            .insert(String::from(prefix), vec![String::from(suffix)]);
                    }
                }
                self.prefix.push(String::from(prefix));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_word_count() {
        let mut a: Vec<String> = vec![];
        a.push(String::from("hello world"));
        a.push(String::from("this is a"));
        a.push(String::from("test"));
        let b: usize = 6;
        let c: usize = super::get_word_count(&a);
        assert_eq!(b, c);
    }
}
