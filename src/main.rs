#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

fn main() {
    println!("Hello, world!");
}

fn same_necklace(s1: &str, s2: &str) -> bool {
    s1 == s2
}

#[cfg(test)]
mod tests {
    use super::*;
    #[quickcheck]
    fn test_works() -> bool {
        true
    }

    #[quickcheck]
    fn matches_the_same_necklace() -> bool {
        same_necklace("ed", "ed")
    }

    #[quickcheck]
    fn fails_on_different_necklaces() -> bool {
        !same_necklace("a", "b")
    }

    #[quickcheck]
    fn always_fails_on_different_necklaces(word: String) -> bool {
        let mut word2 = word.clone();
        word2.push('a');
        !same_necklace(&word, &word2)
    }

    #[quickcheck]
    fn matches_on_out_of_order_word(word: String) -> bool {
        let mut word2 = word.clone();
        let a = word2.pop();
        match a {
            Some (x) => { word2.push(x) },
            None => {}
        }

        same_necklace(&word, &word2)
    }
}
