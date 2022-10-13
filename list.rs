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

fn main() {
  let mut list = List::<i32>::new();
  list.push(10);
  list.push(3);
  list.push(21);
  println!("{:?}", list);
}