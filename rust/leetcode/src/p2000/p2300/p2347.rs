use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn best_hand(ranks: Vec<i32>, suits: Vec<char>) -> String {
        let mut flush = true;
        for i in 1..suits.len() {
            if suits[i - 1] != suits[i] {
                flush = false;
                break;
            }
        }
        if flush {
            return "Flush".to_string();
        }
        let mut m = HashMap::new();
        let mut max_count = 0;
        for x in &ranks {
            m.entry(*x)
                .and_modify(|c| {
                    *c += 1;
                    if *c > max_count {
                        max_count = *c
                    }
                })
                .or_insert(1);
            if max_count == 3 {
                return "Three of a Kind".to_string();
            }
        }
        if max_count == 2 {
            return "Pair".to_string();
        }
        "High Card".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn best_hand_test() {
        assert_eq!(
            Solution::best_hand(vec![1, 2, 3, 4, 5], vec!['a', 'a', 'a', 'a', 'a']),
            "Flush".to_string()
        );
        assert_eq!(
            Solution::best_hand(vec![4, 4, 2, 4, 4], vec!['d', 'a', 'a', 'b', 'c']),
            "Three of a Kind".to_string()
        );
        assert_eq!(
            Solution::best_hand(vec![10, 10, 2, 12, 9], vec!['a', 'b', 'c', 'a', 'd']),
            "Pair".to_string()
        );
        assert_eq!(
            Solution::best_hand(vec![10, 5, 2, 12, 9], vec!['a', 'b', 'c', 'a', 'd']),
            "High Card".to_string()
        );
    }
}
