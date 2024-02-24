// https://leetcode.cn/problems/best-time-to-buy-and-sell-stock/description/
// https://leetcode.cn/problems/house-robber/description/
// https://leetcode.cn/problems/climbing-stairs/solutions/2560716/jiao-ni-yi-bu-bu-si-kao-dong-tai-gui-hua-7zm1/

// 递推方程：dp[i] = max(dp[i-1], dp[i-2]+nums[i]);
pub fn rob(nums: Vec<i32>) -> i32 {
    let mut dp = [0; 2];
    for num in nums {
        dp = [dp[1], (dp[0] + num).max(dp[1])];
    }
    dp[1]
}

pub fn rob_a(nums: Vec<i32>) -> i32 {
    if nums.len() == 1 { return nums[0]; }
    let mut prev_prev = nums[0];
    let mut prev = nums[0].max(nums[1]);
    for i in 2 .. nums.len() {
        let next = prev.max(prev_prev + nums[i]);
        prev_prev = prev;
        prev = next;
    }
    prev
}



// O(2^n)
#[allow(dead_code)]
fn climb_stairs_a(n: i32) -> i32 {
    fn dfs(i:usize) -> i32 {
        if i <=1 {
            return 1;
        }
        dfs(i-1) + dfs(i-2)
    }
    dfs(n as usize)
}

// O(n)
#[allow(dead_code)]
fn climb_stairs_b(n:i32) -> i32 {
    let n = n as usize;
    let mut f = vec![0; n+1];
    f[0] = 1;
    f[1] = 1;
    for i in 2..=n{
        f[i] = f[i-1] + f[i-2];
    }
    f[n]
}

// O(n)
// i 没被用
#[allow(dead_code)]
fn climb_stairs_c(n:i32) -> i32{
    let mut f0 = 1;
    let mut f1 = 1;
    for i in 2..=n {
        let new_f = f1 + f0;
        f0 = f1;
        f1 = new_f;
    }
    f1
}

#[allow(dead_code)]
fn max_profit(prices: Vec<i32>) -> i32 {

}

#[cfg(test)]
mod dy_tests {
    use super::*;

    #[test]
    fn climb_stairs() {
        assert_eq!(climb_stairs_a(2), 2);
        assert_eq!(climb_stairs_a(3), 3);

        assert_eq!(climb_stairs_b(2), 2);
        assert_eq!(climb_stairs_b(3), 3);
    }
}