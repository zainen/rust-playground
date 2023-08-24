#[cfg(test)]
mod tests {

  use my_proc_macro::function_to_string;
  use ai_functions:: ai_function;

  const _OUTPUT: &str = "";

  #[ai_function]
  fn another_ai_function(_whatever_param: &str) {
    /// great function from crates.io lib
    /// giving ai purpose
    /// with structure

    println!("{}", OUTPUT);
  }

  #[function_to_string]
  fn some_function_for_ai(_whatever_param: &str) {
    /// great functions
    /// giving ai purpose
    /// with structure

    println!("{}", OUTPUT);
  }

  #[test]
  fn test_proc_macro() {

    let x: &str = some_function_for_ai("some_input");
    let y: &str = another_ai_function("some_input");
    dbg!(x);
    dbg!(y);

  }
}