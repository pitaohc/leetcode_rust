impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let mut right:i32 = nums.iter().sum();
        let mut left:i32 = 0;
        for (i,&x) in nums.iter().enumerate(){
            right -= x;
            if left == right {
                return i as _;
            }
            left += x;
        }
        -1
    }
}
struct Solution;
#[cfg(test)]
mod tests{
    use crate::Solution;

    #[test]
    fn normal(){
        let nums = vec![1,7,3,6,5,6];
        let ans = 3;
        let result = Solution::pivot_index(nums);
        assert_eq!(result,ans);
    }

    #[test]
    fn left(){
        let nums = vec![2,1,-1];
        let ans = 0;
        let result = Solution::pivot_index(nums);
        assert_eq!(result,ans);
    }
    #[test]
    fn right(){
        let nums = vec![-1,1,2];
        let ans = 2;
        let result = Solution::pivot_index(nums);
        assert_eq!(result,ans);
    }

    #[test]
    fn no(){
        let nums = vec![5,6];
        let ans = -1;
        let result = Solution::pivot_index(nums);
        assert_eq!(result,ans);
    }

    #[test]
    fn single(){
        let nums = vec![1];
        let ans = 0;
        let result = Solution::pivot_index(nums);
        assert_eq!(result,ans);
    }
}

fn main() {

}