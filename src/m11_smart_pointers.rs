#[cfg(test)]
#[allow(dead_code)]
mod tests {
  use std::rc::{Rc, Weak};
  use std::cell::RefCell;

  #[test]
  #[allow(dead_code, unused_variables)]
  fn test_box_smart_pointer() {
    
    #[derive(Debug)]
    struct Node {
      id: u32,
      next: Option<Box<Node>>
    }

    let nodes = Box::new(
      Node { id: 0, next: Some( 
        Box::new(Node { id: 1, next: Some(Box::new(
          Node { id: 2, next: None}
        ))})
      )}
    );
    dbg!(nodes);
  }

  #[test]
  #[allow(dead_code, unused_variables)]
  fn test_reference_counter() {
    let x = Rc::new(RefCell::new(50));
    dbg!(&x);
    let y = Rc::clone(&x);
    *x.borrow_mut() = 70;
    dbg!(y.borrow());
    dbg!(x.borrow());
  }

  #[derive(Debug, Clone)]
  struct House {
    address_number: u16,
    street: String,
    furniture: RefCell<Vec<Rc<Furnature>>>
  }

  #[derive(Debug)]
  struct Furnature {
    id: String,
    description: String,
    house: Weak<House>
  }

  #[test]
  fn test_refcell_pointers() {

    
    let house_1 = Rc::new(House {
      address_number: 123,
      street: "something".to_string(),
      furniture: RefCell::new(vec!())
    });
    let table = Rc::new(Furnature {
      id: "table".to_string(),
      description: "table".to_string(),
      house: Rc::downgrade(&house_1)
    });
    let desk = Rc::new(Furnature {
      id: "desk".to_string(),
      description: "office desk".to_string(),
      house: Rc::downgrade(&house_1)
    });

    house_1.furniture.borrow_mut().push(Rc::clone(&table));
    house_1.furniture.borrow_mut().push(Rc::clone(&desk));

    dbg!(house_1);
  }
}