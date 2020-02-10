use std::cell::RefCell;
use std::rc::Rc;

// >> Linked List
//
// Upsides
// ・Low overhead allocation per item.
// ・Item count is only limited by heap memory.
// ・Mutation while iterating is possible.
// ・A direction is strictly enforced -- there is no going back.
// ・Efficient append, insert operations -- compared to an array(no shifting required)
//
// Downsides
// ・Indexing is inefficient.
// ・Iteration in general involves a lot of jumping around on the heap.
// ・Reversing a list is very inefficient.
//
// A transaction log is great use case for a linked list. They often grow to
// unexpected size and indexing is not required.
// For real-world use cases, it's better to use Rust's standard library linked List.
//

#[derive(Clone, Debug)]
struct Node {
    value: String,
    next: SingleLink,
}

impl Node {
    fn new(v: String) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            value: v,
            next: None,
        }))
    }
}

type SingleLink = Option<Rc<RefCell<Node>>>;

#[derive(Debug)]
struct TransactionLog {
    head: SingleLink,
    tail: SingleLink,
    pub length: u64,
}

impl TransactionLog {
    pub fn new_empty() -> TransactionLog {
        TransactionLog {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn append(&mut self, v: String) {
        let new = Node::new(v);
        match self.tail.take() {
            Some(old) => old.borrow_mut().next = Some(new.clone()),
            None => self.head = Some(new.clone())
        };
        self.length += 1;
        self.tail = Some(new);
    }

    pub fn pop(&mut self) -> Option<String> {
        // Option::map() は Some の中の値にクロージャを適用して、結果を Some で
        // 包み直して返す. もし None に適用すると None を返す.
        // ※ and_then() はクロージャの中で None を返すことができる.
        // ※ map() や and_then() は入力と出力に Option<T> なので数珠つなぎに記述
        //   でき、このような関数をコンビネータという.
        self.head.take().map(|head| {
            if let Some(next) = head.borrow_mut().next.take() {
                self.head = Some(next);
            } else {
                self.tail.take();
            }
            self.length -= 1;
            Rc::try_unwrap(head)
                .ok()
                .expect("Something is terribly wrong")
                .into_inner()
                .value
        })
    }
}


#[test]
fn test_linked_list() {
    let mut log = TransactionLog::new_empty();
    println!("log = {:?}", log);

    println!("\n=== append");
    log.append("this is log 1...".to_string());
    log.append("Hello, I'm log 2...".to_string());
    log.append("Hello, I'm log 3...".to_string());
    println!("log = {:#?}", log);

    println!("\n=== pop");
    let l = log.pop();
    println!("l = {:#?}", l);
    println!("log = {:#?}", log);
}
