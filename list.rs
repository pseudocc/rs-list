use std::fmt;

type Pointer<T> = Option<Box<Node<T>>>;

pub struct List<T> {
  head: Pointer<T>
}

struct Node<T> {
  value: T,
  next: Pointer<T>
}

impl<T> List<T> {
  pub fn new() -> Self {
    List { head: None }
  }

  pub fn push(&mut self, value: T) {
    let new_node = Box::new(Node {
      value, next: self.head.take()
    });
    self.head = Some(new_node);
  }

  pub fn pop(&mut self) -> Option<T> {
    let first = self.head.take();
    match first {
      None => None,
      Some(boxed_node) => {
        self.head = boxed_node.next;
        Some(boxed_node.value)
      }
    }
  }
}

pub struct ListIter<T> {
  list: List<T>
}

impl<T> Iterator for ListIter<T> {
  type Item = T;

  fn next(&mut self) -> Option<Self::Item> {
    self.list.pop()
  }
}

pub struct ListRefIter<'a, T> {
  next: &'a Pointer<T>
}

impl<'a, T> Iterator for ListRefIter<'a, T> {
  type Item = &'a T;

  fn next(&mut self) -> Option<Self::Item> {
    match self.next {
      None => None,
      Some(boxed_node) => {
        self.next = &boxed_node.next;
        Some(&boxed_node.value)
      }
    }
  }
}

impl<T> IntoIterator for List<T> {
  type Item = T;
  type IntoIter = ListIter<T>;

  fn into_iter(self) -> Self::IntoIter {
    ListIter { list: self }
  }
}

impl<'a, T> IntoIterator for &'a List<T> {
  type Item = &'a T;
  type IntoIter = ListRefIter<'a, T>;

  fn into_iter(self) -> Self::IntoIter {
    ListRefIter { next: &self.head }
  }
}

impl<T: fmt::Display> fmt::Display for List<T> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut iter = self.into_iter();
    let head = iter.next();
    if let Some(value) = head {
      // append to the formatter with a '?'
      write!(f, "{}", value)?;
      for value in iter {
        write!(f, " -> {}", value)?;
      }
      write!(f, "")
    } else {
      write!(f, "None")
    }
  }
}

impl<T> Drop for List<T> {
  fn drop(&mut self) {
    while let Some(_) = self.pop() {}
  }
}

fn main() {
  let mut list = List::<i32>::new();
  list.push(10);
  list.push(3);
  list.push(21);
  // fmt::Display
  println!("{}", list);
  // into_iter for &'a List<T>
  for value in &list {
    println!("{}", value);
  }
  // into_iter for List<T>
  // values will be moved, list is no longer accessible.
  for value in list {
    println!("{}", value);
  }
  /*
   * error[E0382]: borrow of moved value: `list`
   * println!("{}", list);
  **/
}
