#[derive(Copy, Clone, Debug)]
pub struct YahtzeeArray {
    data: [u8; 5],
}

impl YahtzeeArray {
    pub fn new() -> YahtzeeArray { YahtzeeArray { data: [0; 5] } }
    pub fn from(nums: [u8; 5]) -> YahtzeeArray { YahtzeeArray { data: nums }}
}

pub fn yahtzee_upper(numbers: YahtzeeArray) -> u8 {
    let mut arr: [u8; 6] = [0; 6];
    for datum in numbers.data.iter() {
        arr[usize::from(*datum) - 1] += *datum;
    }

    let mut max = 0;
    for val in arr.iter() {
        max = if *val > max { *val } else { max }
    }

    max
}

#[cfg(test)]
mod tests {
    use quickcheck::{Arbitrary, Gen};

    use super::*;

    impl Arbitrary for YahtzeeArray {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            YahtzeeArray {
                data: [u8::arbitrary(g) % 6 + 1,
                    u8::arbitrary(g) % 6 + 1,
                    u8::arbitrary(g) % 6 + 1,
                    u8::arbitrary(g) % 6 + 1,
                    u8::arbitrary(g) % 6 + 1]
            }
        }
    }

    #[quickcheck]
    fn function_runs(nums: YahtzeeArray) -> bool {
        yahtzee_upper(nums);
        true
    }

    #[quickcheck]
    fn works(nums: YahtzeeArray) -> bool {
        yahtzee_upper(nums);

        true
    }

    #[test]
    fn works_against_test_data() {
        assert_eq!(yahtzee_upper(YahtzeeArray::from([2, 3, 5, 5, 6])),10);
        assert_eq!(yahtzee_upper(YahtzeeArray::from([1, 1, 1, 1, 3])), 4);
        assert_eq!(yahtzee_upper(YahtzeeArray::from([1, 1, 1, 3, 3])), 6);
        assert_eq!(yahtzee_upper(YahtzeeArray::from([1, 2, 3, 4, 5])), 5);
        assert_eq!(yahtzee_upper(YahtzeeArray::from([6, 6, 6, 6, 6])),30);
    }
}