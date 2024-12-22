use std::collections::HashMap;
impl Solution {
    fn get_weight(x: i32, weights: &mut HashMap<i32, i32>) -> i32 {
        if let Some(weight) = weights.get(&x) {
            return *weight;
        }
        let weight = if x == 1 {
            0
        } else if x & 1 == 1 {
            Self::get_weight(x * 3 + 1, weights) + 1
        } else {
            Self::get_weight(x / 2, weights) + 1
        };
        weights.insert(x,weight);
        weight
    }
    pub fn get_kth(lo: i32, hi: i32, k: i32) -> i32 {
        let mut weights: HashMap<i32, i32> = HashMap::new();
        let mut numbers = (lo..=hi).map(|x| (Self::get_weight(x,&mut weights),x)).collect::<Vec<_>>();
        numbers.sort();
        numbers[(k - 1) as usize].1
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn case1() {
        let (lo, hi, k) = (12, 15, 2);
        let ans = 13;
        let res = Solution::get_kth(lo, hi, k);
        assert_eq!(ans, res);
    }
    #[test]
    fn case2() {
        let (lo, hi, k) = (7, 11, 4);
        let ans = 7;
        let res = Solution::get_kth(lo, hi, k);
        assert_eq!(ans, res);
    }
}
