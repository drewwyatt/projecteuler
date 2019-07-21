// Problem 1: Multiples of 3 and 5
// If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
// Find the sum of all the multiples of 3 or 5 below 1000.

fn sum_of_multiples(below: i32) -> i32 {
  let mut sum = 0;
  for i in 0..below {
    if i % 3 == 0 {
      sum = sum + i;
    } else if i % 5 == 0 {
      sum = sum + i;
    }
  }

  sum
}

pub fn main() -> i32 {
  sum_of_multiples(1000)
}

#[test]
fn it_can_find_multiples_below_10() {
  assert_eq!(sum_of_multiples(10), 23)
}
