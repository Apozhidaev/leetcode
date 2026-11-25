// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
  pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
      let mut l1 = l1;
      let mut l2 = l2;
      let mut dummy = Box::new(ListNode::new(0));
      let mut curr = &mut dummy;
      let mut carry = 0;

      while l1.is_some() || l2.is_some() || carry != 0 {
          let x = l1.as_ref().map_or(0, |n| n.val);
          let y = l2.as_ref().map_or(0, |n| n.val);
          let sum = x + y + carry;
          carry = sum / 10;
          curr.next = Some(Box::new(ListNode::new(sum % 10)));
          curr = curr.next.as_mut().unwrap();
          l1 = l1.and_then(|n| n.next);
          l2 = l2.and_then(|n| n.next);
      }

      dummy.next
  }
}
