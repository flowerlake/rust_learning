use crate::enum_test::List::{Cons, Nil};

pub enum List {
    Cons(u32, Box<List>),
    Nil,
}

impl List {
    pub fn new() -> List {
        Nil
    }

    fn postpend(self, element: u32) -> List {
        Cons(element, Box::new(self))
    }

    pub fn prepend(&mut self, element: u32) {
        // code by chatgpt, 这里每次新建一个Cons
        let new_node = Cons(element, Box::new(std::mem::replace(self, List::Nil)));
        *self = new_node;
    }

    pub fn len(&self) -> u32 {
        match self {
            Cons(_, ref element) => { 1 + element.len() }
            Nil => { 0 }
        }
    }

    pub fn stringify(&self) -> String {
        match self {
            Cons(header, ref element) => {
                // `format!` 和 `print!` 类似，但返回的是一个堆分配的字符串，而不是打印结果到控制台上
                format!("{}, {}", header, element.stringify())
            }
            Nil => {
                format!("")
            }
        }
    }
}

