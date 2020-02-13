use std::cell::RefCell;
use std::rc::Rc;

// >> Doubly Linked List
//
// Upsides/Downsides
// ・Almost all features are the same as Linked List.
// ・There is some comment in Rust source code ...
//    >> It is almost always better to use `Vec` or `VecDeque`
//       because array-based containers are generally faster, more memory
//       efficient, and make better use of CPU cache.
//

#[derive(Clone, Debug)]
struct Node {
    value: String,
    next: Link,
    prev: Link,
}

impl Node {
    fn new(v: String) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            value: v,
            next: None,
            prev: None,
        }))
    }
}

type Link = Option<Rc<RefCell<Node>>>;

#[derive(Clone, Debug)]
struct BetterTransactionLog {
    head: Link,
    tail: Link,
    pub length: u64,
}

impl BetterTransactionLog {
    pub fn new_empty() -> BetterTransactionLog {
        BetterTransactionLog {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn append(&mut self, v: String) {
        let new = Node::new(v);
        match self.tail.take() {
            Some(old) => {
                old.borrow_mut().next = Some(new.clone());
                new.borrow_mut().prev = Some(old);
            },
            None => self.head = Some(new.clone()),
        };
        self.length += 1;
        self.tail = Some(new);
    }

    // get element from Head
    pub fn pop(&mut self) -> Option<String> {
        self.head.take().map(|head| {
            if let Some(next) = head.borrow_mut().next.take() {
                next.borrow_mut().prev = None;
                self.head = Some(next);
            } else {
                self.tail.take();
            }
            self.length -= 1;
            // head: Rc<RefCell<Node>>
            Rc::try_unwrap(head)
                .ok()
                .expect("something is terribly wrong")
                // RefCell
                .into_inner()
                .value
        })
    }

    pub fn iter(&self) -> ListIterator {
        ListIterator::new(self.head.clone())
    }

    pub fn back_iter(self) -> ListIterator {
        ListIterator::new(self.tail)
    }
}

impl IntoIterator for BetterTransactionLog {
    type Item = String;
    type IntoIter = ListIterator;

    fn into_iter(self) -> Self::IntoIter {
        ListIterator::new(self.head)
    }
}

pub struct ListIterator {
    current: Link,
}

impl ListIterator {
    fn new(start: Link) -> ListIterator {
        ListIterator { current: start }
    }
}

impl Iterator for ListIterator {
    type Item = String;
    fn next(&mut self) -> Option<String> {
        // 一時退避
        let current = &self.current;
        // result はこの関数の戻り値にも使われるし、ListIterator の状態にも使われる
        let mut result = None;
        // 状態更新
        self.current = match current {
            Some(ref current) => {
                let current = current.borrow();
                result = Some(current.value.clone());
                current.next.clone()
            },
            None => None
        };
        result
    }
}

impl DoubleEndedIterator for ListIterator {
    fn next_back(&mut self) -> Option<String> {
        let current = &self.current;
        let mut result = None;
        self.current = match current {
            Some(ref current) => {
                let current = current.borrow();
                result = Some(current.value.clone());
                current.prev.clone()
            },
            None => None
        };
        result
    }
}

#[test]
fn test_doubly_linked_list() {
    let mut log = BetterTransactionLog::new_empty();

    log.append("this is log 1...".to_string());
    log.append("Hello, I'm log 2...".to_string());
    log.append("Hello, I'm log 3...".to_string());
    log.append("Hello, I'm log 4...".to_string());
    log.append("Hello, I'm log 5...".to_string());
    log.append("Hello, I'm log 6...".to_string());
    log.append("Hello, I'm log 7...".to_string());

    let mut log_iter = log.into_iter();
    //println!("{:?}", &log_iter.next());
    //println!("{:?}", &log_iter.next());
    //println!("{:?}", &log_iter.next());
    println!("{:?}", &log_iter.next_back());

}
