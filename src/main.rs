pub mod problems;

fn show_answer_for<F>(number: i32, solution: F) where F: Fn() -> i32 {
  let answer = solution();
  println!("Solution for problem {}: {}", number, answer);
}

fn main() {
  show_answer_for(1, problems::problem_1::main);
  show_answer_for(2, problems::problem_2::main);
}
