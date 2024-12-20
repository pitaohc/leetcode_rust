
impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;
        intervals.sort_unstable_by(|a,b| a[0].cmp(&b[0]));
        let mut merged :Vec<Vec<i32>>= Vec::new();
        for interval in intervals{
            if let Some(last) = merged.last_mut(){
                if interval[0] <= last[1]{
                    last[1] = std::cmp::max(last[1],interval[1]);
                }else{
                    merged.push(interval);
                }
            }else{
                merged.push(interval);
            }
        }
        merged
    }
}
struct Solution;

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn case1() {
        let intervals = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
        let ans = vec![vec![1,6],vec![8,10],vec![15,18]];
        let res = Solution::merge(intervals);
        assert_eq!(res,ans);
    }
}
