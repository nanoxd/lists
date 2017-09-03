pub struct Iter<'a, T: 'a> {
  next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
  type Item = &'a T;
  fn next(&mut self) -> Option<Self::Item> {
    self.next.map(|node| {
      self.next = node.next.as_ref().map(|node| &**node);
      &node.elem
    })
  }
}

pub struct IterMut<'a, T: 'a> {
  next: Option<&'a mut Node<T>>,
}

impl <'a, T> Iterator for IterMut<'a, T> {
  type Item = &'a mut T;

  fn next(&mut self) -> Option<Self::Item> {
    self.next.take().map(|node| {
      self.next = node.next.as_mut().map(|node| &mut **node);
      &mut node.elem
    })
  }
}

pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
  type Item = T;

  fn next(&mut self) -> Option<Self::Item> {
    self.0.pop()
  }
}

pub struct List<T> {
  head: Link<T>,
}

impl<T> List<T> {
  pub fn new() -> Self {
    List { head: None }
  }

  pub fn push(&mut self, elem: T) {
    let new_node = Box::new(Node {
      elem,
      next: self.head.take(),
    });

    self.head = Some(new_node)
  }

  pub fn pop(&mut self) -> Option<T> {
    self.head.take().map(|node| {
      let node = *node;
      self.head = node.next;
      node.elem
    })
  }

  pub fn peek(&self) -> Option<&T> {
    self.head.as_ref().map(|node| &node.elem)
  }

  pub fn peek_mut(&mut self) -> Option<&mut T> {
    self.head.as_mut().map(|node| &mut node.elem)
  }

  pub fn iter<'a>(&'a self) -> Iter<'a, T> {
    Iter {
      next: self.head.as_ref().map(|node| &**node),
    }
  }

  pub fn iter_mut(&mut self) -> IterMut<T> {
    IterMut {
      next: self.head.as_mut().map(|node| &mut **node)
    }
  }

  pub fn into_iter(self) -> IntoIter<T> {
    IntoIter(self)
  }
}

impl<T> Drop for List<T> {
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

struct Node<T> {
  elem: T,
  next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;


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

  #[test]
  fn peek() {
    let mut list = List::new();
    assert_eq!(list.peek(), None);
    assert_eq!(list.peek_mut(), None);
    list.push(1);
    list.push(2);
    list.push(3);

    assert_eq!(list.peek(), Some(&3));
    assert_eq!(list.peek_mut(), Some(&mut 3));
  }

  #[test]
  fn into_iter() {
    let mut list = List::new();
    list.push(1);
    list.push(2);
    list.push(3);

    let mut iter = list.into_iter();
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(1));
  }

  #[test]
  fn iter() {
    let mut list = List::new();
    list.push(1);
    list.push(2);
    list.push(3);

    let mut iter = list.iter();
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&1));
  }

  #[test]
  fn iter_mut() {
      let mut list = List::new();
      list.push(1); list.push(2); list.push(3);

      let mut iter = list.iter_mut();
      assert_eq!(iter.next(), Some(&mut 3));
      assert_eq!(iter.next(), Some(&mut 2));
      assert_eq!(iter.next(), Some(&mut 1));
  }
}
