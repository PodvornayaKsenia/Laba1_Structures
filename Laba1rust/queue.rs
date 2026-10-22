pub struct Node {
    pub data: String,
    pub next: Option<Box<Node>>,
}

pub struct Queue {
    pub head: Option<Box<Node>>,
    pub tail: *mut Node,
}

impl Queue {
    pub fn new() -> Self { Queue { head: None, tail: std::ptr::null_mut() } }

    pub fn push(&mut self, val: String) {
        let mut new_node = Box::new(Node { data: val, next: None });
        let ptr = &mut *new_node as *mut Node;
        unsafe {
            if self.tail.is_null() {
                self.head = Some(new_node);
            } else {
                (*self.tail).next = Some(new_node);
            }
            self.tail = ptr;
        }
    }

    pub fn pop(&mut self) -> Option<String> {
        self.head.take().map(|node| {
            self.head = node.next;
            if self.head.is_none() {
                self.tail = std::ptr::null_mut();
            }
            node.data
        })
    }

    pub fn print(&self) {
        let mut cur = &self.head;
        print!("[");
        while let Some(node) = cur {
            print!("{} ", node.data);
            cur = &node.next;
        }
        println!("]");
    }
}
