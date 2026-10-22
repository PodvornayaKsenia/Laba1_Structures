pub struct Array {
    pub data: *mut String,
    pub size: usize,
    pub capacity: usize,
}

impl Array {
    pub fn new() -> Self {
        let cap = 2;
        let data = unsafe {
            let layout = std::alloc::Layout::array::<String>(cap).unwrap();
            std::alloc::alloc(layout) as *mut String
        };
        Array { data, size: 0, capacity: cap }
    }

    pub fn m_init(&mut self, cap: usize) {
        self.size = 0;
        self.capacity = cap;
        unsafe {
            self.data = std::alloc::alloc(std::alloc::Layout::array::<String>(cap).unwrap()) as *mut String;
        }
    }

    fn resize(&mut self) {
        let new_cap = if self.capacity == 0 { 2 } else { self.capacity * 2 };
        let new_data = unsafe {
            std::alloc::alloc(std::alloc::Layout::array::<String>(new_cap).unwrap()) as *mut String
        };
        for i in 0..self.size {
            unsafe {
                std::ptr::write(new_data.add(i), std::ptr::read(self.data.add(i)));
            }
        }
        unsafe {
            std::alloc::dealloc(self.data as *mut u8, std::alloc::Layout::array::<String>(self.capacity).unwrap());
        }
        self.data = new_data;
        self.capacity = new_cap;
    }

    pub fn push(&mut self, val: String) {
        if self.capacity == 0 { self.m_init(2); }
        if self.size == self.capacity { self.resize(); }
        unsafe {
            std::ptr::write(self.data.add(self.size), val);
        }
        self.size += 1;
    }

    pub fn insert(&mut self, index: usize, val: String) {
        if index > self.size { return; }
        if self.size == self.capacity { self.resize(); }
        unsafe {
            for i in (index..self.size).rev() {
                std::ptr::write(self.data.add(i + 1), std::ptr::read(self.data.add(i)));
            }
            std::ptr::write(self.data.add(index), val);
        }
        self.size += 1;
    }

    pub fn get(&self, index: usize) -> Option<String> {
        if index >= self.size { return None; }
        unsafe {
            Some(std::ptr::read(self.data.add(index)))
        }
    }

    pub fn remove(&mut self, index: usize) {
        if index >= self.size { return; }
        unsafe {
            std::ptr::drop_in_place(self.data.add(index));
            for i in index..self.size - 1 {
                std::ptr::write(self.data.add(i), std::ptr::read(self.data.add(i + 1)));
            }
        }
        self.size -= 1;
    }

    pub fn replace(&mut self, index: usize, val: String) {
        if index >= self.size { return; }
        unsafe {
            std::ptr::write(self.data.add(index), val);
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn print(&self) {
        if self.size == 0 {
            println!("[Массив пуст]");
            return;
        }
        print!("[");
        for i in 0..self.size {
            unsafe {
                print!("{}{}", std::ptr::read(self.data.add(i)), if i == self.size - 1 { "" } else { ", " });
            }
        }
        println!("]");
    }
}

