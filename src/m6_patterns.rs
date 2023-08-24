#[cfg(test)]
mod test {
  enum Message {
    Quit,
    ChangeColour( i32, i32, i32),
    Move { x: i32, y: i32 },
    Write(String)
  }

  fn process_message(msg: Message) {
    match msg {
      Message::Quit => {
        println!("I quit");
      },
      Message::ChangeColour(red, green, blue) => {
        println!("RED {} GREEN {} BLUE {}", red, green, blue);
      },
      Message::Move { x, y } => {
        println!("X {} Y {}", x, y);
      },
      Message::Write(str) => {
        println!("{}", str);
      }
    }
  }



  #[test]
  fn test_large_enum () {
    let my_quit: Message = Message::Quit;
    process_message(my_quit);
    let my_colour: Message = Message::ChangeColour(10, 25, 35);
    process_message(my_colour);
    let my_position: Message = Message::Move { x: 255, y: 255 };
    process_message(my_position);
    let my_write: Message = Message::Write("send help".to_string());
    process_message(my_write);
  }

  #[test]
  fn test_match_literals() {
    let number = 20;

    let res: &str = match number {
      1 => "first",
      2 | 3 | 5 | 7 | 15 | 20 => "found",
      _ => "something else"
    };
    println!("res is {}", res)
  }

  #[test]
  fn test_match_option () {
    let some_num: Option<i32> = Some(10);
    let _prob_none: Option<i32> = None;

    let res = match some_num {
      Some(i) => i,
      None => {
        panic!("there as a problem");
      }
    };
    println!("{}", res);
  }

  #[test]
  fn test_match_let_some () {
    let some_num: Option<i32> = Some(10);

    let my_int: i32 = if let Some(i) = some_num {
      i
    } else {
      panic!("there was a problem");
    };

    println!("{}", my_int);
  }

  #[test]
  fn test_match_result () {
    let some_res: Result<i32, &str> = Ok(10);
    let _some_err: Result<i32, &str>  = Err("error");

    let res = match some_res {
      Ok(val) => val,
      Err(e) => panic!("{}", e)
    };

    println!("{}", res);
  }

  #[test]
  fn test_match_guard() {
    let pair = (2, -2);
    match pair {
      (x, y) if x == y => println!("match"),
      (x, y) if x != y => println!("doesnt match"),
      (_, y) if y == 2 => println!("y is 2"),
      _ => println!("not bothered")
    };
  }

  #[test]
  fn test_match_struct() {
    struct Location {
      x: i32, 
      y: i32
    }

    let location = Location {x: 0, y: 20};
    
    match location {
      Location {x: _, y: 0} => println!("y is on the axis"),
      Location {x: 0, y: _ } => println!("x is on the axis"),
      Location {x: _, y: _} => println!("y and x are not on the axis")
    };
  }

}