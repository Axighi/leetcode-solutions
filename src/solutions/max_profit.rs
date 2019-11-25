pub struct Solution {}

impl Solution {
  #[allow(dead_code)]
  pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut profit: i32 = 0;
    if prices.len() == 0 || prices.len() == 1 {
      return 0i32;
    }

    let mut buy_index: usize = 0;
    let mut sell_index: usize = 0;

    while buy_index <= prices.len() - 2 {
      sell_index = buy_index + 1;
      while sell_index <= prices.len() - 1 {
        let temp_profit = prices[sell_index] - prices[buy_index];
        if temp_profit > profit {
          profit = temp_profit
        }
        sell_index += 1;
      }

      buy_index += 1;
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
