use rand::prelude::*;
use clap::{Clap, crate_version, crate_authors};
use std::fs;
use std::path::Path;
use std::str;

mod markov;
mod random;

use markov::MarkovGenerator;

#[derive(Clap)]
#[clap(version = crate_version!(), author = crate_authors!())]
struct Opts {
    file_path: String,
    #[clap(short = 'w', long, about = "Total generated word count. Default: 10")]
    word_count: Option<usize>,
    #[clap(short = 'n', long, about = "N value for generation. Default: 2")]
    n_count: Option<usize>,
    #[clap(short = 'o', long, about = "Out file path. If missing, will output to stdout instead.")]
    out_path: Option<String>
}

fn main() {
    let opts: Opts = Opts::parse();
    let n_count = opts.n_count.unwrap_or(2);
    let word_count = opts.word_count.unwrap_or(10);
    let input = Path::new(&opts.file_path);
    let mut rng = StdRng::from_entropy();
    let mut mk: MarkovGenerator = MarkovGenerator::new(&mut rng);
    let fdata = fs::read(input).unwrap();
    mk.parse(str::from_utf8(&fdata[..]).unwrap(), n_count);
    let gen = mk.generate(word_count as i32);
    match opts.out_path {
        Some(out) => {
            let out_path = Path::new(&out);
            fs::write(out_path, &gen).unwrap();
        }
        None => {
            println!("{}", gen);
        }
    }
}
