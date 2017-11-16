extern crate rand;
extern crate spectral;

use rand::{thread_rng, sample};
use spectral::prelude::*;

mod names;

pub fn all() -> Vec<&'static str> {
    names::get_names()
}

pub fn random(n: usize) -> Vec<&'static str> {
    let mut rng = thread_rng();
    let all_names = names::get_names();

    sample(&mut rng, all_names, n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_all() {
        let all_names = all();

        assert_that(&all_names).has_length(93);
    }

    #[test]
    fn get_random() {
        let num = 3 as usize;
        let random_names = random(num);

        assert_that(&random_names).has_length(num);
        
        let all_names = all();

        for name in &random_names {
            assert_that(&all_names).contains(name);
        }
    }
}
