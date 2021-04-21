#[derive(Debug)]
struct MyCircularQueue {
    items: Vec<i32>,
    start: usize,
    end: usize,
}

impl MyCircularQueue {
    fn new(k: i32) -> Self {
        MyCircularQueue {
            items: vec![-1; k as usize],
            start: 0,
            end: (k - 1) as usize,
        }
    }

    fn en_queue(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }

        self.end += 1;
        self.end = self.end % self.items.capacity();
        self.items[self.end] = value;
        true
    }

    fn de_queue(&mut self) -> bool {
        println!("{:?}", self);
        if self.is_empty() {
            return false;
        }
        self.items[self.start] = -1;
        self.start += 1;
        self.start = self.start % self.items.capacity();

        true
    }

    fn front(&self) -> i32 {
        self.items[self.start]
    }

    fn rear(&self) -> i32 {
        self.items[self.end]
    }

    fn is_empty(&self) -> bool {
        self.start == (self.end + 1) % self.items.capacity() && self.front() == -1
    }

    fn is_full(&self) -> bool {
        self.items[(self.end + 1) % self.items.capacity()] != -1
    }
}

#[test]
fn it_is_empty() {
    let q = MyCircularQueue::new(10);
    assert!(q.is_empty());
}

#[test]
fn it_is_full() {
    let mut q = MyCircularQueue::new(1);
    q.en_queue(10);
    assert!(q.is_full());
}

#[test]
fn it_handles_sequence() {
    let mut q = MyCircularQueue::new(3);
    assert_eq!(true, q.en_queue(1));
    assert_eq!(true, q.en_queue(2));
    assert_eq!(true, q.en_queue(3));
    assert_eq!(false, q.en_queue(4));
    assert_eq!(3, q.rear());
    assert_eq!(true, q.is_full());
    assert_eq!(true, q.de_queue());
    assert_eq!(true, q.en_queue(4));
    assert_eq!(4, q.rear());
}
