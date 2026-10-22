pub struct Node {
    pub data: String,
    pub next: Option<Box<Node>>,
    pub prev: *mut Node, 
}

pub struct DoubleList {
    pub head: Option<Box<Node>>,
    pub tail: *mut Node,
}

impl DoubleList {
    pub fn new() -> Self { DoubleList { head: None, tail: std::ptr::null_mut() } }

    pub fn push_front(&mut self, val: String) {
        let mut new_node = Box::new(Node { data: val, next: self.head.take(), prev: std::ptr::null_mut() });
        let ptr = &mut *new_node as *mut Node;
        if let Some(ref mut head) = new_node.next {
            head.prev = ptr;
        } else {
            self.tail = ptr;
        }
        self.head = Some(new_node);
    }

    pub fn push_back(&mut self, val: String) {
        let mut new_node = Box::new(Node { data: val, next: None, prev: self.tail });
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

    pub fn insert_before(&mut self, target: &str, val: String) {
        if self.head.as_ref().map(|h| h.data == target).unwrap_or(false) {
            self.push_front(val);
            return;
        }
        unsafe {
            let mut cur = self.head.as_mut();
            while let Some(node) = cur {
                if let Some(ref mut next) = node.next {
                    if next.data == target {
                        let mut new_node = Box::new(Node { data: val, next: node.next.take(), prev: node.as_mut() as *mut Node });
                        let ptr = &mut *new_node as *mut Node;
                        if let Some(ref mut n) = new_node.next {
                            n.prev = ptr;
                        }
                        node.next = Some(new_node);
                        return;
                    }
                }
                cur = node.next.as_mut();
            }
        }
    }

    pub fn insert_after(&mut self, target: &str, val: String) {
        unsafe {
            let mut cur = self.head.as_mut();
            while let Some(node) = cur {
                if node.data == target {
                    let old_next = node.next.take();
                    let mut new_node = Box::new(Node { data: val, next: old_next, prev: node.as_mut() as *mut Node });
                    let ptr = &mut *new_node as *mut Node;
                    if let Some(ref mut n) = new_node.next {
                        n.prev = ptr;
                    } else {
                        self.tail = ptr;
                    }
                    node.next = Some(new_node);
                    return;
                }
                cur = node.next.as_mut();
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<String> {
        self.head.take().map(|mut node| {
            self.head = node.next.take();
            if let Some(ref mut head) = self.head {
                head.prev = std::ptr::null_mut();
            } else {
                self.tail = std::ptr::null_mut();
            }
            node.data
        })
    }

    pub fn pop_back(&mut self) -> Option<String> {
        unsafe {
            if self.tail.is_null() { return None; }
            let tail_node = Box::from_raw(self.tail);
            let val = tail_node.data.clone();
            self.tail = tail_node.prev;
            if self.tail.is_null() {
                self.head = None;
            } else {
                (*self.tail).next = None;
            }
            Some(val)
        }
    }

    pub fn remove_by_value(&mut self, val: &str) {
        if self.head.as_ref().map(|h| h.data == val).unwrap_or(false) {
            self.pop_front();
            return;
        }
        unsafe {
            let mut cur = self.head.as_mut();
            while let Some(node) = cur {
                if let Some(ref mut next) = node.next {
                    if next.data == val {
                        let mut to_delete = node.next.take().unwrap();
                        if let Some(ref mut n) = to_delete.next {
                            n.prev = node.as_mut() as *mut Node;
                        } else {
                            self.tail = node.as_mut() as *mut Node;
                        }
                        node.next = to_delete.next.take();
                        return;
                    }
                }
                cur = node.next.as_mut();
            }
        }
    }

    pub fn print(&self) {
        let mut cur = &self.head;
        print!("[");
        while let Some(node) = cur {
            print!("{} <-> ", node.data);
            cur = &node.next;
        }
        println!("NULL]");
    }
}
