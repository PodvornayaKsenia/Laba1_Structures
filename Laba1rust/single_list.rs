pub struct Node {
    pub data: String,
    pub next: Option<Box<Node>>,
}

pub struct SingleList {
    pub head: Option<Box<Node>>,
}

impl SingleList {
    pub fn new() -> Self { SingleList { head: None } }

    pub fn push_front(&mut self, val: String) {
        self.head = Some(Box::new(Node { data: val, next: self.head.take() }));
    }

    pub fn push_back(&mut self, val: String) {
        let mut cur = &mut self.head;
        while let Some(ref mut node) = cur {
            cur = &mut node.next;
        }
        *cur = Some(Box::new(Node { data: val, next: None }));
    }

    pub fn insert_after(&mut self, target: &str, val: String) {
        let mut cur = &mut self.head;
        while let Some(ref mut node) = cur {
            if node.data == target {
                let new_node = Box::new(Node { data: val, next: node.next.take() });
                node.next = Some(new_node);
                return;
            }
            cur = &mut node.next;
        }
    }

    pub fn insert_before(&mut self, target: &str, val: String) {
        if let Some(ref mut head) = self.head {
            if head.data == target {
                self.push_front(val);
                return;
            }
        }
        let mut cur = &mut self.head;
        while let Some(ref mut node) = cur {
            let is_target = if let Some(ref next) = node.next {
                next.data == target
            } else {
                false
            };

            if is_target {
                let new_node = Box::new(Node { data: val, next: node.next.take() });
                node.next = Some(new_node);
                return;
            }
            cur = &mut node.next;
        }
    }

    pub fn pop_front(&mut self) -> Option<String> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.data
        })
    }

    pub fn pop_back(&mut self) -> Option<String> {
        unsafe {
            let mut cur = &mut self.head as *mut Option<Box<Node>>;
            if (*cur).is_none() { return None; }

            loop {
                let node_opt = &mut *cur;
                if let Some(node) = node_opt {
                    if node.next.is_none() {
                        let boxed_node = (*cur).take().unwrap();
                        return Some(boxed_node.data);
                    }
                    cur = &mut node.next as *mut Option<Box<Node>>;
                } else {
                    break;
                }
            }
            None
        }
    }

    pub fn remove_by_value(&mut self, val: &str) {
        unsafe {
            let mut cur = &mut self.head as *mut Option<Box<Node>>;
            loop {
                let node_opt = &mut *cur;
                if let Some(node) = node_opt {
                    if node.data == val {
                        let mut boxed_node = (*cur).take().unwrap();
                        *cur = boxed_node.next.take();
                        return;
                    }
                    cur = &mut node.next as *mut Option<Box<Node>>;
                } else {
                    break;
                }
            }
        }
    }

    pub fn search(&self, val: &str) -> bool {
        let mut cur = &self.head;
        while let Some(node) = cur {
            if node.data == val { return true; }
            cur = &node.next;
        }
        false
    }

    pub fn print(&self) {
        let mut cur = &self.head;
        print!("[");
        while let Some(node) = cur {
            print!("{} -> ", node.data);
            cur = &node.next;
        }
        println!("NULL]");
    }
}
