impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len();
        while left < right {
            let mid = (right - left) / 2 + left;
            if nums[mid] >= target {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        left as i32
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn find_mid() {
        let nums = vec![1, 3, 5, 6];
        let target = 5;
        let ans = 2;
        let res = Solution::search_insert(nums, target);
        assert_eq!(res, ans);
    }
    #[test]
    fn find_left() {
        let nums = vec![1, 3, 5, 6];
        let target = 1;
        let ans = 0;
        let res = Solution::search_insert(nums, target);
        assert_eq!(res, ans);
    }
    #[test]
    fn find_right() {
        let nums = vec![1, 3, 5, 6];
        let target = 6;
        let ans = 3;
        let res = Solution::search_insert(nums, target);
        assert_eq!(res, ans);
    }
    #[test]
    fn not_find_mid() {
        let nums = vec![1, 3, 5, 6];
        let target = 2;
        let ans = 1;
        let res = Solution::search_insert(nums, target);
        assert_eq!(res, ans);
    }
    #[test]
    fn not_find_left() {
        let nums = vec![1, 3, 5, 6];
        let target = 0;
        let ans = 0;
        let res = Solution::search_insert(nums, target);
        assert_eq!(res, ans);
    }
    #[test]
    fn not_find_right() {
        let nums = vec![1, 3, 5, 6];
        let target = 7;
        let ans = 4;
        let res = Solution::search_insert(nums, target);
        assert_eq!(res, ans);
    }
    #[test]
    fn test() {
        let nums = vec![1, 3];
        let target = 0;
        let ans = 0;
        let res = Solution::search_insert(nums, target);
        assert_eq!(res, ans);
    }
}
