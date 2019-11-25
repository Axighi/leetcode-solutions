pub struct Solution {}

impl Solution {
  #[allow(dead_code)]
  pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut min_price = std::i32::MAX;
    let mut profit: i32 = 0;

    for p in prices {
      if p < min_price {
        min_price = p;
      } else if p - min_price > profit {
        profit = p - min_price;
      }
    }

    profit
  }
}

#[test]
fn hooyah() {
  assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
  assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
  assert_eq!(Solution::max_profit(vec![]), 0);
  assert_eq!(Solution::max_profit(vec![1]), 0);
}
