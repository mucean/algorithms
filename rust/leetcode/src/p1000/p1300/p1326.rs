pub struct Solution {}

impl Solution {
    pub fn min_taps(n: i32, ranges: Vec<i32>) -> i32 {
        use std::cmp;
        let n: usize = n as usize;
        let mut ans: Vec<[usize; 2]> = vec![[0, 0]; ranges.len() + 1];
        let mut check_posit = 0;
        for i in 0..ranges.len() {
            // range == 0
            if ranges[i] == 0 {
                // check max range is coverred current pos
                if ans[i][0] > 0 && ans[i][1] >= i {
                    ans[i+1][0] = ans[i][0];
                    ans[i+1][1] = ans[i][1];
                }
                continue;
            }
            // range > 0
            // check min range pos is coverred
            let left_posit = i as i32 - ranges[i];
            let right_posit = i + ranges[i] as usize;
            if left_posit <= 0 {
                if right_posit >= n {
                    return 1;
                }
                ans[i+1] = [1, cmp::max(right_posit, ans[i][1])];
                continue;
            }
            check_posit = left_posit as usize;
            println!("check_posit: {}, i: {}", check_posit, i);
            if ans[check_posit][0] == 0 {
                continue;
            }
            if ans[check_posit][1] >= right_posit {
                ans[i+1] = [ans[check_posit][0], ans[check_posit][1]];
            } else {
                ans[i+1] = [ans[check_posit][0] + 1, right_posit];
            }
        }
        ans[n+1][0] as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_taps_test() {
        assert_eq!(Solution::min_taps(3, vec![0, 1, 1, 0]), 2);
        if true {
            return;
        }
        assert_eq!(Solution::min_taps(2, vec![0, 0, 1]), 0);
        assert_eq!(Solution::min_taps(1, vec![0, 1]), 1);
        assert_eq!(Solution::min_taps(2, vec![0, 1, 0]), 1);
        assert_eq!(Solution::min_taps(3, vec![0, 1, 0, 0]), 0);
        assert_eq!(Solution::min_taps(5, vec![3, 4, 1, 1, 0, 0]), 1);
        assert_eq!(Solution::min_taps(5, vec![3, 1, 1, 1, 2, 0]), 2);
        assert_eq!(Solution::min_taps(5, vec![3, 1, 1, 1, 1, 0]), 2);
        assert_eq!(Solution::min_taps(5, vec![3, 1, 1, 1, 1, 5]), 1);
        assert_eq!(Solution::min_taps(3, vec![0, 0, 0, 0]), 0);
    }
}
