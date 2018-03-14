extern crate rand;
#[cfg(test)]
extern crate spectral;

mod names;

use rand::thread_rng;

/// Get all character names
///
/// # Example
///
/// ```rust,ignore
/// let all_names = starwars-names::all();
///
/// assert_that(&all_names).has_length(93);
/// ```
pub fn all() -> Vec<&'static str> {
    names::NAMES.to_owned()
}

/// Get radom names
///
/// # Example
///
/// ```rust,ignore
/// let num = 3 as usize;
/// let random_names = starwars-names::random(num);
///
/// assert_that(&random_names).has_length(num);
/// ```
pub fn random(n: usize) -> Vec<&'static str> {
    let mut rng = thread_rng();

    rand::seq::sample_slice(&mut rng, names::NAMES, n)
}

#[cfg(test)]
mod tests {
    use super::{all, random};
    use spectral::prelude::*;
    
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
