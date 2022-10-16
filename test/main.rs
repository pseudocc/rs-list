extern crate list;
use list::List;

#[test]
fn push_and_pop() {
  let first = 11;
  let second = 29;
  let third = 92;

  let mut list = List::<i32>::new();
  assert_eq!(list.pop(), None);

  list.push(first);
  assert_eq!(list.pop(), Some(first));
  assert_eq!(list.pop(), None);

  list.push(second);
  list.push(third);
  assert_eq!(list.pop(), Some(third));
  assert_eq!(list.pop(), Some(second));
  assert_eq!(list.pop(), None);
}

#[test]
fn fmt_display() {
  let mut display;
  let mut list = List::<i32>::new();

  display = format!("{}", list);
  assert_eq!(display, "None");

  list.push(8);
  list.push(15);
  list.push(95);

  display = format!("{}", list);
  assert_eq!(display, "95 -> 15 -> 8");
}

#[test]
fn into_iter() {
  let mut list = List::<i32>::new();
  let nums = [10, 16, 22];

  for i in (0..3).rev() {
    list.push(nums[i]);
  }

  // into_iter for &'a List<T>
  let mut i = 0;
  for value in &list {
    assert_eq!(value.clone(), nums[i]);
    i += 1;
  }

  // into_iter for List<T>
  let mut i = 0;
  for value in list {
    assert_eq!(value, nums[i]);
    i += 1;
  }
}
