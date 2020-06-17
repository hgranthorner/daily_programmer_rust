
pub fn same_necklace(s1: String, s2: String) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    let double_word = s1.repeat(2);

    if double_word.contains(&s2) {
        return true;
    }

    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[quickcheck]
    fn matches_the_same_necklace() -> bool {
        same_necklace(String::from("ed"), String::from("ed"))
    }

    #[quickcheck]
    fn fails_on_different_necklaces() -> bool {
        !same_necklace(String::from("a"), String::from("b"))
    }

    #[quickcheck]
    fn always_fails_on_different_necklaces(word: String) -> bool {
        let mut word2 = word.clone();
        word2.push('a');
        !same_necklace(word, word2)
    }

    #[quickcheck]
    fn matches_on_out_of_order_word(word: String) -> bool {
        let mut word2 = word.clone();
        let a = word2.pop();
        match a {
            Some(x) => word2.insert(0, x),
            None => {}
        }

        same_necklace(word, word2)
    }
}
