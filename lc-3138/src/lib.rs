impl Solution {
    pub fn min_anagram_length(s: String) -> i32 {
        let n = s.len();
        let check = |k: usize| ->bool{
            let mut count0 = vec![0;26];
            for i in (0..n).step_by(k){
                let mut count1 = vec![0;26];
                for j in i..i+k{
                    let index = s.as_bytes()[j] as usize - b'a' as usize;
                    count1[index] +=1;
                }
                if i>0 && count0 != count1{
                    return false;
                } 
                count0 = count1;
            }
            true
        };
        for i in 1..=n{
            if n % i == 0 && check(i) {
                return i as i32;
            }
        }
        0
    }
}
struct Solution;

#[cfg(test)]
mod tests{
    use crate::Solution;

    #[test]
    fn case1(){
        let s = String::from("abba");
        let ans = 2;
        let res = Solution::min_anagram_length(s);
        assert_eq!(res,ans);
    }
    #[test]
    fn case2(){
        let s = String::from("abcd");
        let ans = 4;
        let res = Solution::min_anagram_length(s);
        assert_eq!(res,ans);
    }
    #[test]
    fn case3(){
        let s = String::from("a");
        let ans = 1;
        let res = Solution::min_anagram_length(s);
        assert_eq!(res,ans);
    }

}