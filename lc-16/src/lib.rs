impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        let mut result = nums[0] + nums[1] + nums[2];
        nums.sort_unstable();
        for i in 0..nums.len()-2{
            let (mut l,mut r) = (i+1,nums.len()-1);
            while l < r {
                let sum = nums[i] + nums[l] + nums[r];
                if target == sum {
                    return sum;
                }else if (target - result).abs() > (target - sum).abs(){
                    result = sum;
                }
                if sum > target{
                    r -= 1;
                }else{
                    l+=1;
                }
            }
        }
        result
    }
}
struct Solution;

#[cfg(test)]
mod tests{
    use crate::Solution;

    #[test]
    fn case1(){
        let nums = vec![-1,2,1,-4];
        let target = 1;
        let ans = 2;
        let res = Solution::three_sum_closest(nums,target);
        assert_eq!(res,ans);
    }
    #[test]
    fn case2(){
        let nums = vec![0,0,0];
        let target = 1;
        let ans = 0;
        let res = Solution::three_sum_closest(nums,target);
        assert_eq!(res,ans);
    }
}