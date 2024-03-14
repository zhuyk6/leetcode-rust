struct Solution;

#[allow(unused)]
impl Solution {
    pub fn xor_beauty(nums: Vec<i32>) -> i32 {
        let s = nums.iter().fold(0, |acc, e| acc ^ *e);
        let t = nums.iter().fold(0, |acc, e| acc ^ !(*e));
        println!("s = {s:032b}");
        println!("t = {t:032b}");
        s & (if nums.len() % 2 == 0 {
            s & t
        } else {
            ((s ^ nums[0]) & (t ^ nums[0])) ^ nums[0]
        })
    }
}

#[test]
fn test1() {
    let nums = vec![1000000000, 1, 1];
    assert_eq!(Solution::xor_beauty(nums), 1000000000);
}

#[test]
fn test2() {
    let nums = vec![5];
    assert_eq!(Solution::xor_beauty(nums), 5);
}
