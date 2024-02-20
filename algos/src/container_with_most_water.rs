#[allow(dead_code)]

// https://leetcode.cn/problems/container-with-most-water/description/
// O(n) & two pointers
fn max_area(height: Vec<i32>) -> i32{
    let mut ans = 0;
    let mut left = 0;
    let mut right = height.len() - 1;
    while left < right {
        let area = ((right - left) as i32) * height[left].min(height[right]);
        ans = ans.max(area);
        if height[left] < height[right]{
            left +=1;
        } else {
            right -=1;
        }
    }
    ans
}

#[allow(dead_code)]
// https://leetcode.cn/problems/trapping-rain-water/solutions/1974340/zuo-liao-nbian-huan-bu-hui-yi-ge-shi-pin-ukwm/
// https://www.bilibili.com/video/BV1Qg411q7ia/?vd_source=0289608f552ecea26ba6cc2a4d567bd5
fn trap_two_pointer(height: Vec<i32>) -> i32{
    let mut ans = 0;
    let mut left = 0;
    let mut right = height.len() -1;
    let mut pre_max = 0;
    let mut suf_max = 0;
    while left < right {
        pre_max = pre_max.max(height[left]);
        suf_max = suf_max.max(height[right]);
        if pre_max < suf_max {
            ans += pre_max - height[left];
            left +=1;
        } else {
            ans += suf_max - height[right];
            right -=1;
        };
    }
    ans
}
#[allow(dead_code)]
// 前后缀分解 O(n)
fn trap_pre_suf(height: Vec<i32>) -> i32{
        let n = height.len();
        let mut pre_max = vec![0; n]; // pre_max[i] 表示从 height[0] 到 height[i] 的最大值
        pre_max[0] = height[0];
        for i in 1..n {
            pre_max[i] = pre_max[i - 1].max(height[i]);
        }

        let mut suf_max = vec![0; n]; // suf_max[i] 表示从 height[i] 到 height[n-1] 的最大值
        suf_max[n - 1] = height[n - 1];
        for i in (0..n - 1).rev() {
            suf_max[i] = suf_max[i + 1].max(height[i]);
        }

        let mut ans = 0;
        for i in 0..n {
            ans += pre_max[i].min(suf_max[i]) - height[i]; // 累加每个水桶能接多少水
        }
        ans
    
}

#[allow(dead_code)]
// https://www.bilibili.com/video/BV1VN411J7S7/?vd_source=0289608f552ecea26ba6cc2a4d567bd5
fn trap_stack(height: Vec<i32>) -> i32{
    let mut ans = 0;
        let mut st: Vec<usize> = Vec::new();
        for (i, &h) in height.iter().enumerate() {
            while !st.is_empty() && h >= height[st[st.len() - 1]] {
                let bottom_h = height[st.pop().unwrap()];
                if st.is_empty() {
                    break;
                }
                let left = st[st.len() - 1];
                let dh = height[left].min(h) - bottom_h;
                ans += dh * ((i - left - 1) as i32);
            }
            st.push(i);
        }
        ans
}

#[cfg(test)]
mod max_area_tests {
    use super::*;

    #[test]
    fn max_area_base() {
        let height = vec![1,8,6,2,5,4,8,3,7];
        assert_eq!(max_area(height), 49);
    }

    #[test]
    fn trap_base() {
        let height = vec![0,1,0,2,1,0,1,3,2,1,2,1];
        assert_eq!(trap_two_pointer(height.clone()), 6);
        assert_eq!(trap_pre_suf(height.clone()), 6);
        assert_eq!(trap_stack(height.clone()), 6);
    }
}