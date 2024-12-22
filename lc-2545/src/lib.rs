

impl Solution {
    pub fn sort_the_students(score: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut score = score.clone();
        score.sort_unstable_by(|l,r|{
            r[k as usize].cmp(&l[k as usize])
        });
        score
    }
}
struct Solution;

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn case1() {
        let score = vec![vec![10, 6, 9, 1], vec![7, 5, 11, 2], vec![4, 8, 3, 15]];
        let k = 2;
        let ans = vec![vec![7, 5, 11, 2], vec![10, 6, 9, 1], vec![4, 8, 3, 15]];
        let res = Solution::sort_the_students(score, k);
        assert_eq!(res, ans);
    }
    #[test]
    fn case2() {
        let score = vec![vec![3, 4], vec![5, 6]];
        let k = 0;
        let ans = vec![vec![5, 6], vec![3, 4]];
        let res = Solution::sort_the_students(score, k);
        assert_eq!(res, ans);
    }
}
