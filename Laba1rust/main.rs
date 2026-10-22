mod stack;
mod queue;
mod array;
mod single_list;
mod double_list;

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::process;

enum Container {
    S(stack::Stack),
    F(single_list::SingleList),
    L(double_list::DoubleList),
    Q(queue::Queue),
    M(array::Array),
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        eprintln!("Использование: {} --file <файл> --query '<команда>'", args[0]);
        process::exit(1);
    }
    let filename = &args[2];
    let query = &args[4];

    let parts: Vec<&str> = query.split_whitespace().collect();
    if parts.len() < 2 { return; }
    let command = parts[0];
    let name = parts[1];

    let mut db: HashMap<String, Container> = HashMap::new();

    if let Ok(file) = File::open(filename) {
        for line in io::BufReader::new(file).lines() {
            if let Ok(l) = line {
                let tokens: Vec<&str> = l.split_whitespace().collect();
                if tokens.len() < 2 { continue; }
                let t_type = tokens[0];
                let t_name = tokens[1];
                match t_type {
                    "S" => {
                        let mut s = stack::Stack::new();
                        for &v in &tokens[2..] { s.push(v.to_string()); }
                        db.insert(t_name.to_string(), Container::S(s));
                    },
                    "F" => {
                        let mut s = single_list::SingleList::new();
                        for &v in &tokens[2..] { s.push_back(v.to_string()); }
                        db.insert(t_name.to_string(), Container::F(s));
                    },
                    "L" => {
                        let mut s = double_list::DoubleList::new();
                        for &v in &tokens[2..] { s.push_back(v.to_string()); }
                        db.insert(t_name.to_string(), Container::L(s));
                    },
                    "Q" => {
                        let mut s = queue::Queue::new();
                        for &v in &tokens[2..] { s.push(v.to_string()); }
                        db.insert(t_name.to_string(), Container::Q(s));
                    },
                    "M" => {
                        let mut s = array::Array::new();
                        for &v in &tokens[2..] { s.push(v.to_string()); }
                        db.insert(t_name.to_string(), Container::M(s));
                    },
                    _ => {}
                }
            }
        }
    }

    let mut changed = false;

    if !db.contains_key(name) {
        if let Some(c) = command.chars().next() {
            match c {
                'S' => { db.insert(name.to_string(), Container::S(stack::Stack::new())); },
                'F' => { db.insert(name.to_string(), Container::F(single_list::SingleList::new())); },
                'L' => { db.insert(name.to_string(), Container::L(double_list::DoubleList::new())); },
                'Q' => { db.insert(name.to_string(), Container::Q(queue::Queue::new())); },
                'M' => { db.insert(name.to_string(), Container::M(array::Array::new())); },
                _ => {}
            }
        }
    }

    if let Some(container) = db.get_mut(name) {
        match command {
            "PRINT" => match container {
                Container::S(s) => s.print(),
                Container::F(s) => s.print(),
                Container::L(s) => s.print(),
                Container::Q(s) => s.print(),
                Container::M(s) => s.print(),
            },
            
            "SPUSH" => if let Container::S(s) = container { 
                if parts.len() >= 3 {
                    s.push(parts[2].to_string()); 
                    println!("{}", parts[2]); 
                    changed = true; 
                }
            },
            "SPOP" => if let Container::S(s) = container { 
                if let Some(v) = s.pop() { println!("{}", v); } 
                changed = true; 
            },
            
            "FPUSH" => if let Container::F(s) = container { 
                if parts.len() >= 4 {
                    if parts[2] == "front" { s.push_front(parts[3].to_string()); println!("{}", parts[3]); }
                    else if parts[2] == "back" { s.push_back(parts[3].to_string()); println!("{}", parts[3]); }
                    else if parts[2] == "after" && parts.len() >= 5 { s.insert_after(parts[3], parts[4].to_string()); println!("{}", parts[4]); }
                    else if parts[2] == "before" && parts.len() >= 5 { s.insert_before(parts[3], parts[4].to_string()); println!("{}", parts[4]); }
                    changed = true;
                }
            },
            "FGET" => if let Container::F(s) = container { s.print(); },
            "FDEL" => if let Container::F(s) = container { 
                if parts.len() >= 3 {
                    if parts[2] == "front" { s.pop_front(); } 
                    else if parts[2] == "back" { s.pop_back(); } 
                    else if parts[2] == "after" && parts.len() >= 4 { s.remove_by_value(parts[3]); }
                    else if parts[2] == "before" && parts.len() >= 4 { s.remove_by_value(parts[3]); }
                    else { s.remove_by_value(parts[2]); }
                    changed = true;
                }
            },
            "FIND" => if let Container::F(s) = container { 
                if parts.len() >= 3 {
                    if s.search(parts[2]) { println!("TRUE"); } else { println!("FALSE"); } 
                }
            },

            "LPUSH" => if let Container::L(s) = container { 
                if parts.len() >= 4 {
                    if parts[2] == "front" { s.push_front(parts[3].to_string()); println!("{}", parts[3]); }
                    else if parts[2] == "back" { s.push_back(parts[3].to_string()); println!("{}", parts[3]); }
                    else if parts[2] == "after" && parts.len() >= 5 { s.insert_after(parts[3], parts[4].to_string()); println!("{}", parts[4]); }
                    else if parts[2] == "before" && parts.len() >= 5 { s.insert_before(parts[3], parts[4].to_string()); println!("{}", parts[4]); }
                    changed = true;
                }
            },
            "LGET" => if let Container::L(s) = container { s.print(); },
            "LDEL" => if let Container::L(s) = container { 
                if parts.len() >= 3 {
                    if parts[2] == "front" { s.pop_front(); } 
                    else if parts[2] == "back" { s.pop_back(); } 
                    else if parts[2] == "after" && parts.len() >= 4 { s.remove_by_value(parts[3]); }
                    else if parts[2] == "before" && parts.len() >= 4 { s.remove_by_value(parts[3]); }
                    else { s.remove_by_value(parts[2]); }
                    changed = true;
                }
            },

            "QPUSH" => if let Container::Q(s) = container { 
                if parts.len() >= 3 {
                    s.push(parts[2].to_string()); 
                    println!("{}", parts[2]); 
                    changed = true; 
                }
            },
            "QPOP" => if let Container::Q(s) = container { 
                if let Some(v) = s.pop() { println!("{}", v); } 
                changed = true; 
            },
            "QGET" => if let Container::Q(s) = container { s.print(); },

            "MPUSH" => if let Container::M(s) = container { 
                if parts.len() >= 3 {
                    s.push(parts[2].to_string()); 
                    println!("{}", parts[2]); 
                    changed = true; 
                }
            },
            "MGET" => if let Container::M(s) = container { 
                if parts.len() >= 3 {
                    if let Ok(idx) = parts[2].parse::<usize>() { 
                        if let Some(v) = s.get(idx) { println!("{}", v); } 
                    } 
                }
            },
            "MLEN" => if let Container::M(s) = container { println!("{}", s.len()); },
            "MINSERT" => if let Container::M(s) = container { 
                if parts.len() >= 4 {
                    if let Ok(idx) = parts[2].parse::<usize>() { 
                        s.insert(idx, parts[3].to_string()); 
                        println!("{}", parts[3]); 
                        changed = true; 
                    } 
                }
            },
            "MDEL" => if let Container::M(s) = container { 
                if parts.len() >= 3 {
                    if let Ok(idx) = parts[2].parse::<usize>() { s.remove(idx); changed = true; } 
                }
            },
            "MSET" => if let Container::M(s) = container { 
                if parts.len() >= 4 {
                    if let Ok(idx) = parts[2].parse::<usize>() { 
                        s.replace(idx, parts[3].to_string()); 
                        println!("{}", parts[3]); 
                        changed = true; 
                    } 
                }
            },
            
            _ => println!("Команда не реализована"),
        }
    }

    if changed {
        if let Ok(mut file) = File::create(filename) {
            for (key, container) in &db {
                let mut line = String::new();
                match container {
                    Container::S(s) => { 
                        line.push_str(&format!("S {} ", key)); 
                        let mut cur = &s.top;
                        while let Some(n) = cur { line.push_str(&format!("{} ", n.data)); cur = &n.next; }
                    },
                    Container::F(s) => { 
                        line.push_str(&format!("F {} ", key)); 
                        let mut cur = &s.head;
                        while let Some(n) = cur { line.push_str(&format!("{} ", n.data)); cur = &n.next; }
                    },
                    Container::L(s) => { 
                        line.push_str(&format!("L {} ", key)); 
                        let mut cur = &s.head;
                        while let Some(n) = cur { line.push_str(&format!("{} ", n.data)); cur = &n.next; }
                    },
                    Container::Q(s) => { 
                        line.push_str(&format!("Q {} ", key)); 
                        let mut cur = &s.head;
                        while let Some(n) = cur { line.push_str(&format!("{} ", n.data)); cur = &n.next; }
                    },
                    Container::M(s) => { 
                        line.push_str(&format!("M {} ", key)); 
                        for i in 0..s.size { 
                            unsafe { line.push_str(&format!("{} ", *s.data.add(i))); }
                        } 
                    },
                }
                let _ = writeln!(file, "{}", line);
            }
        }
    }
}
