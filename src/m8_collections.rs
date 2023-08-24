#[cfg(test)]
mod test {
  use std::collections::{HashMap, HashSet};

  #[test]
  fn test_hashmap() {
    let person_1: &str = "alice";
    let person_2: &str = "bob";

    let mut results_hm: HashMap<&str, u32> = HashMap::new();
    results_hm.insert(person_1, 55);
    results_hm.insert(person_2, 50);

    let test_score: Option<&u32> = results_hm.get(person_1);
    dbg!(test_score.unwrap());
  }

  #[test]
  fn test_hashset() {
    let mut names_hs: HashSet<&str> = HashSet::new();
    names_hs.insert("alice");
    names_hs.insert("bob");
    names_hs.insert("john");

    if names_hs.contains("alice") {
      println!("contains")
    }
  }
}