#[cfg(test)]
mod test {
  #[derive(Debug)]
  #[allow(dead_code)]
  struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
  }

  impl User {
    fn increment_signin_count(&mut self) {
      self.sign_in_count += 1;
    }

    fn change_email(&mut self, new_email: &str) {
      self.email = String::from(new_email);
    }
  }

  fn change_name(user: &mut User, new_username: &str) {
    user.username = String::from(new_username);
  }
  
 #[test]
  fn test_structs() {
    let mut user_1: User = User {
      username: String::from("Someone"),
      email: String::from("someemail"),
      sign_in_count: 20,
      active: false
    };
    dbg!(&user_1);


    change_name(&mut user_1, "else");

    user_1.change_email("new_email");
    user_1.increment_signin_count();

    dbg!(user_1);
  }
}