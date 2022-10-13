use std::fmt;

type Pointer<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub struct List<T> {
  head: Pointer<T>
}

#[derive(Debug)]
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
  next: Pointer<T>
}

impl<T> Iterator for ListIter<T> {
  type Item = T;

  fn next(&mut self) -> Option<Self::Item> {
    let current = self.next.take();
    match current {
      None => None,
      Some(boxed_node) => {
        self.next = boxed_node.next;
        Some(boxed_node.value)
      }
    }
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
    ListIter { next: self.head }
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

fn main() {
  let mut list = List::<i32>::new();
  list.push(10);
  list.push(3);
  list.push(21);
  println!("{}", list);
  for value in &list {
    println!("{}", value);
  }
  for value in list {
    println!("{}", value);
  }
}