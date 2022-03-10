#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

pub fn test(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>
) -> Option<Box<ListNode>> {
  tests(l1, l2, 0)
}

pub fn tests(
  l1: Option<Box<ListNode>>,
  l2: Option<Box<ListNode>>,
  mut carry: i32,
) -> Option<Box<ListNode>> {
  if l1.is_none() && l2.is_none() && carry == 0 {
    None
  } else {
    Some(
      Box::new(
        ListNode {
          next: tests(
            l1.and_then(|x| {
              carry += x.val;
              x.next
            }),
            l2.and_then(|x| {
              carry += x.val;
              x.next
            }),
            carry / 10
          ),
          val: carry % 10
        }
      )
    )
  }
}