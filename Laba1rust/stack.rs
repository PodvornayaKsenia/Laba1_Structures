pub struct Node {
    pub data: String,
    pub next: Option<Box<Node>>,
}

pub struct Stack {
    pub top: Option<Box<Node>>,
}

impl Stack {
    pub fn new() -> Self { Stack { top: None } }
    
    pub fn push(&mut self, val: String) {
        let new_node = Box::new(Node { data: val, next: self.top.take() });
        self.top = Some(new_node);
    }
    
    pub fn pop(&mut self) -> Option<String> {
        self.top.take().map(|node| {
            self.top = node.next;
            node.data
        })
    }
    
    pub fn print(&self) {
        let mut cur = &self.top;
        print!("[");
        while let Some(node) = cur {
            print!("{} ", node.data);
            cur = &node.next;
        }
        println!("]");
    }
}
