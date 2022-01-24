use std::fmt;

type Result<T, E> = std::result::Result<T, E>;

// Define our error types. These may be customized for our error handling cases.
// Now we will be able to write our own errors, defer to an underlying error
// implementation, or do something in between.
#[derive(Debug, Clone)]
struct BTNodeAddToNoneError;

// Generation of an error is completely separate from how it is displayed.
// There's no need to be concerned about cluttering complex logic with the display style.
//
// Note that we don't store any extra info about the errors. This means we can't state
// which string failed to parse without modifying our types to carry that information.
impl fmt::Display for BTNodeAddToNoneError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "cannot add because BTNode is none")
  }
}


// enum BtNode<T: Copy> {
//   Leaf(T),
//   Branch: {
//     val: T,
//     left: Box<BtNode<T>>,
//     right: Box<BtNode<T>>,
//   }
// }

struct BTNode<T: Copy> {
  val: Option<T>,
  left: Option<Box<BTNode<T>>>,
  right: Option<Box<BTNode<T>>>
}

struct BinaryTree<T: Copy> {
  // root: Option<BTNode<T>>
  root: BTNode<T>
}

impl <T: Copy> BTNode<T> {
  fn new(val: T) -> Self {
    BTNode{
      val: Some(val), 
      left: None,
      right: None
    }
  }

  fn add_left(&mut self, val: T) {
    self.left = Some(Box::new(BTNode::new(val)));
  }

  fn get_left_val(&self) -> Option<T> {
    match &self.left {
      Some(node) => {
        node.val
      },
      None => None
    }
  }

  fn add_right(&mut self, val: T) {
    self.right = Some(Box::new(BTNode::new(val)));
  }

  fn get_right_val(&self) -> Option<T> {
    match &self.right {
      Some(node) => {
        node.val
      },
      None => None
    }
  }
}

impl<T: Copy> BinaryTree<T> {
  fn new() -> Self {
    BinaryTree{ root: BTNode{
      val: None,
      left: None,
      right: None
    }}
  }
}

#[cfg(test)]
mod test {
  use super::BinaryTree;

  #[test]
  fn basics() {
    let mut bt = BinaryTree::<i32>::new();
    
    bt.root.add_left(10);
    assert_eq!(bt.root.get_left_val(), Some(10));


    bt.root.add_right(11);
    assert_eq!(bt.root.get_right_val(), Some(11));



    // match bt.root.right {
      
    // }
    // assert_eq!(bt.root.get_right_val(), Some(12));
  }
}