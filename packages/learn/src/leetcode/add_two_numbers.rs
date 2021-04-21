use std::fmt;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    fn to_vec(&self) -> Vec<i32> {
        let mut node = self;
        let mut items = Vec::new();

        loop {
            let val = node.val;
            items.push(val);
            match &node.next {
                Some(x) => {
                    node = &x;
                }
                None => break,
            }
        }

        items
    }
}

impl fmt::Display for ListNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.to_vec())
    }
}

#[allow(dead_code)]
fn create_list(arr: Vec<i32>) -> Option<Box<ListNode>> {
    arr.into_iter().rev().fold(None, |next, val| {
        let node = ListNode { val, next };
        Some(Box::new(node))
    })
}

#[allow(dead_code)]
fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut i1 = l1;
    let mut i2 = l2;
    let mut carry = 0;
    let mut head = Some(Box::new(ListNode::new(0)));
    let mut curr = head.as_mut();

    while i1 != None || i2 != None {
        let val1 = match &i1 {
            Some(x) => x.val,
            None => 0,
        };
        let val2 = match &i2 {
            Some(x) => x.val,
            None => 0,
        };

        let total = carry + val1 + val2;
        carry = total / 10;

        if let Some(node) = curr {
            node.next = Some(Box::new(ListNode::new(total % 10)));
            curr = node.next.as_mut();
        }

        i1 = match i1 {
            Some(x) => x.next,
            None => None,
        };

        i2 = match i2 {
            Some(x) => x.next,
            None => None,
        };
    }

    if carry > 0 {
        curr.unwrap().next = Some(Box::new(ListNode::new(carry)));
    }

    head.unwrap().next
}

#[test]
fn it_add_two_numbers() {
    let n1 = create_list(vec![2, 4, 3]).unwrap();
    let n2 = create_list(vec![5, 6, 4]).unwrap();

    let l1 = n1.to_vec();
    let l2 = n2.to_vec();

    let result = add_two_numbers(Some(n1), Some(n2));
    println!("{:?} + {:?} = {}", l1, l2, result.unwrap());
}
