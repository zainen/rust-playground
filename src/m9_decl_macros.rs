#[cfg(test)]
mod tests {

  // macro_rules! mad_skills {
  //   // ($x: expr) => {
  //   //   format!("You sent an expression: {}", $x)
  //   // };
  //   ($x: ty) => {
  //     match stringify!($x) {
  //       "i32" => "You sent an i32 type".to_string(),
  //       _ => "Something else".to_string(),
  //     }
  //   };
  // }
  #[macro_export]
  macro_rules!  my_vec {
    ($($x: expr),+ ) => {
      {
        let mut temp_vec = Vec::new();
        $(
          temp_vec.push($x);
        )+
        temp_vec
      }
    };
  }

  #[test]
  fn test_declarative_macro() {

    let x: Vec<i32> = vec!();
    let y: Vec<i32> = my_vec!(1,2,3);
    dbg!(x);
    dbg!(y);


    // let some_var: String = mad_skills!(u8);
    // dbg!(some_var);
  }
}