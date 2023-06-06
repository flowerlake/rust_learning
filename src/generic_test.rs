struct SGen {
    val: u32,
}

struct Generic<T> {
    val: T,
}

impl SGen {
    pub fn get(&self) -> &u32 { &self.val }
}

impl<T> Generic<T> {
    pub fn get(&self) -> &T { &self.val }
}

#[cfg(test)]
mod test {
    use crate::generic_test::{Generic, SGen};

    #[test]
    fn test1() {
        let x = SGen { val: 1 };
        let y = Generic { val: "c" };
        let z = Generic { val: 3 };

        println!("value is {:?}", x.get());
        println!("value of Generic is {:}", y.get());
        println!("value of Generic is {:}", z.get());
    }
}