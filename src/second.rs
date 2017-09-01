use std::mem;

pub struct List {
  head: Link,
}

impl List {
  pub fn new() -> Self {
    List { head: None }
  }

  pub fn push(&mut self, elem: i32) {
    let new_node = Box::new(Node {
      elem,
      next: self.head.take(),
    });

    self.head = Some(new_node)
  }

  pub fn pop(&mut self) -> Option<i32> {
    match self.head.take() {
      None => None,
      Some(boxed_node) => {
        let node = *boxed_node;
        self.head = node.next;
        Some(node.elem)
      }
    }
  }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        // `while let` == "do this thing until this pattern doesn't match"
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
            // boxed_node goes out of scope and gets dropped here;
            // but its Node's `next` field has been set to None
            // so no unbounded recursion occurs.
        }
    }
}

struct Node {
  elem: i32,
  next: Link,
}

type Link = Option<Box<Node>>;

#[cfg(test)]
mod test {
  use super::List;

  #[test]
  fn basics() {
    let mut list = List::new();
    assert_eq!(list.pop(), None);

    list.push(1);
    list.push(2);
    list.push(3);

    assert_eq!(list.pop(), Some(3));
    assert_eq!(list.pop(), Some(2));

    list.push(4);
    list.push(5);

    assert_eq!(list.pop(), Some(5));
    assert_eq!(list.pop(), Some(4));

    // Check exhaustion
    assert_eq!(list.pop(), Some(1));
    assert_eq!(list.pop(), None);
  }
}
