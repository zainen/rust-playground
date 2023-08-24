#[allow(dead_code, unused_variables)]
fn example_0 () {
  let _: &i32;

  let x: i32 = 5;
  _ = &x;
}


#[allow(dead_code, unused_variables)]
fn example_1() {
  let highest_age: i32;

  let alice_age: i32 = 20;

  let bob_age: i32 = 21;
  
  highest_age = largest(&alice_age, &bob_age);

  println!("Highest age is {}", highest_age);

  fn largest<'a, 'b: 'a>(compare_1: &'a i32, compare_2: &'b i32) -> i32 {
    if compare_1 > compare_2 {
      *compare_1
    } else {
      *compare_2
    }
  }
}

#[allow(dead_code, unused_variables)]
fn example_2() {
  let highest_age: &i32;

  let alice_age: i32 = 20;

  let bob_age: i32 = 21;
  
  highest_age = largest(&alice_age, &bob_age);

  println!("Highest age is {}", highest_age);

  fn largest<'a, 'b: 'a>(compare_1: &'a i32, compare_2: &'b i32) -> &'a i32 {
    if compare_1 > compare_2 {
      compare_1
    } else {
      compare_2
    }
  }
}

#[allow(dead_code, unused_variables)]
fn example_3_generics() {
  let highest_age: &i32;

  let alice_age: i32 = 20;

  let bob_age: i32 = 21;
  
  highest_age = largest(&alice_age, &bob_age);

  println!("Highest age is {}", highest_age);

  fn largest<'a, 'b: 'a, T: PartialOrd>(compare_1: &'a T, compare_2: &'b T) -> &'a T {
    if compare_1 > compare_2 {
      compare_1
    } else {
      compare_2
    }
  }
}

#[allow(dead_code, unused_variables)]
fn example_4_struct() {
  let highest_age: &f32;
  let new_value: &f32;

  let alice: Person = Person { name: "alice", points: &50.2 };
  {
    let bob: Person = Person { name: "bob", points: &40.2 };
    new_value = bob.points
  }

  highest_age = largest(alice.points, new_value);

  println!("Highest age is {}", highest_age);

  fn largest<'a, 'b: 'a, T: PartialOrd>(compare_1: &'a T, compare_2: &'b T) -> &'a T {
    if compare_1 > compare_2 {
      compare_1
    } else {
      compare_2
    }
  }

}

#[allow(dead_code)]
struct Person<'p> {
  name: &'p str,
  points: &'p f32
}