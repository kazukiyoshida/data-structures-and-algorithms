use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug)]
struct Node {
    next: Vec<Link>,
    pub offset: u64,
    pub command: String,
}

impl Node {
    fn new(offset: u64, command: String) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            next: vec![],
            offset: offset,
            command: command,
        }))
    }
}

type Link = Option<Rc<RefCell<Node>>>;

struct SkipList {
    head: Link,
    tails: Vec<Link>,
    max_level: usize,
    pub length: u64,
}

impl SkipList {
    pub fn new_empty() -> SkipList {
        SkipList {
            head: None,
            tails: vec![],
            max_level: 0,
            length: 0,
        }
    }

    pub fn append(&mut self, offset: u64, value: String) {
    }
}
