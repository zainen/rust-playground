#[cfg(test)]
mod test {
  
  trait Attacker {
    fn choose_style(&self) -> String;
  }

  #[derive(Debug)]
  #[allow(dead_code)]
  enum Character {
    Warrior,
    Archer,
    Wizard
  }

  impl Attacker for Character {
    fn choose_style(&self) -> String {
        match self {
          Character::Warrior => "Sword".to_string(),
          Character::Archer => "Bow".to_string(),
          Character::Wizard => "Staff".to_string(),
        }
    }
  }

 #[test]
  fn test_traits() {
    let my_character = Character::Archer;
    let chosen_style = my_character.choose_style();
    dbg!(chosen_style);
  }
}