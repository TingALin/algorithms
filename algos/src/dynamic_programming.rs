// https://leetcode.cn/problems/longest-common-subsequence/description/

// https://leetcode.cn/problems/house-robber/description/
#[allow(dead_code)]
// 递推方程：dp[i] = max(dp[i-1], dp[i-2]+nums[i]);
pub fn rob_a(nums: Vec<i32>) -> i32 {
    let mut dp = [0; 2];
    for num in nums {
        dp = [dp[1], (dp[0] + num).max(dp[1])];
    }
    dp[1]
}

#[allow(dead_code)]
pub fn rob_b(nums: Vec<i32>) -> i32 {
    if nums.len() == 1 {
        return nums[0];
    }
    let mut prev_prev = nums[0];
    let mut prev = nums[0].max(nums[1]);
    for i in 2..nums.len() {
        let next = prev.max(prev_prev + nums[i]);
        prev_prev = prev;
        prev = next;
    }
    prev
}

// https://leetcode.cn/problems/climbing-stairs/solutions/2560716/jiao-ni-yi-bu-bu-si-kao-dong-tai-gui-hua-7zm1/
// O(2^n)
#[allow(dead_code)]
fn climb_stairs_a(n: i32) -> i32 {
    fn dfs(i: usize) -> i32 {
        if i <= 1 {
            return 1;
        }
        dfs(i - 1) + dfs(i - 2)
    }
    dfs(n as usize)
}

// O(n)
#[allow(dead_code)]
fn climb_stairs_b(n: i32) -> i32 {
    let n = n as usize;
    let mut f = vec![0; n + 1];
    f[0] = 1;
    f[1] = 1;
    for i in 2..=n {
        f[i] = f[i - 1] + f[i - 2];
    }
    f[n]
}

// O(n)
// i 没被用
#[allow(dead_code)]
fn climb_stairs_c(n: i32) -> i32 {
    let mut f0 = 1;
    let mut f1 = 1;
    for i in 2..=n {
        let new_f = f1 + f0;
        f0 = f1;
        f1 = new_f;
    }
    f1
}

// https://leetcode.cn/problems/best-time-to-buy-and-sell-stock/description/
// O(n)
#[allow(dead_code)]
pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut ans = 0;
    let mut min_price = prices[0];
    for &p in &prices {
        ans = ans.max(p - min_price);
        min_price = min_price.min(p);
    }
    ans
}

// 双指针
pub fn max_profit_two_pointer(prices: Vec<i32>) -> i32 {
    if prices.len() == 1 {
        return 0;
    }
    let mut ans = 0;
    let (mut slow, mut fast) = (0, 1);
    while fast < prices.len() {
        if prices[slow] > prices[fast] {
            slow = fast;
        } else {
            ans = ans.max(prices[fast] - prices[slow]);
        }
        fast += 1;
    }
    ans
}

#[cfg(test)]
mod dy_tests {
    use super::*;

    #[test]
    fn robing_houses() {
        let list_of_house = vec![1, 4, 5, 1, 2, 3];
        assert_eq!(rob_a(list_of_house), 9);
    }

    #[test]
    fn climb_stairs() {
        assert_eq!(climb_stairs_a(2), 2);
        assert_eq!(climb_stairs_a(3), 3);

        assert_eq!(climb_stairs_b(2), 2);
        assert_eq!(climb_stairs_b(3), 3);
    }
}
