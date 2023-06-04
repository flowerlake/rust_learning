// mod enum_test 会查找名为 `enum_test.rs` 或 `enum_test/mod.rs` 的文件，模块的两种方式
mod enum_test;

use enum_test::*;

// 一个名为 `my_mod` 的模块
mod my_mod {
    // 模块中的项默认具有私有的可见性
    fn private_function() {
        println!("called `my_mod::private_function()`");
    }

    // 使用 `pub` 修饰语来改变默认可见性。
    pub fn function() {
        println!("called `my_mod::function()`");
    }

    // 在同一模块中，项可以访问其它项，即使它是私有的。
    pub fn indirect_access() {
        print!("called `my_mod::indirect_access()`, that\n> ");
        private_function();
    }

    // 模块也可以嵌套
    pub mod nested {
        pub fn function() {
            println!("called `my_mod::nested::function()`");
        }

        #[allow(dead_code)]
        fn private_function() {
            println!("called `my_mod::nested::private_function()`");
        }

        // 使用 `pub(in path)` 语法定义的函数只在给定的路径中可见。
        // `path` 必须是父模块（parent module）或祖先模块（ancestor module）
        pub(in crate::my_mod) fn public_function_in_my_mod() {
            print!("called `my_mod::nested::public_function_in_my_mod()`, that\n > ");
            public_function_in_nested()
        }

        // 使用 `pub(self)` 语法定义的函数则只在当前模块中可见。
        pub(self) fn public_function_in_nested() {
            println!("called `my_mod::nested::public_function_in_nested");
        }

        // 使用 `pub(super)` 语法定义的函数只在父模块中可见。
        pub(super) fn public_function_in_super_mod() {
            println!("called my_mod::nested::public_function_in_super_mod");
        }
    }

    pub fn call_public_function_in_my_mod() {
        print!("called `my_mod::call_public_funcion_in_my_mod()`, that\n> ");
        nested::public_function_in_my_mod();
        print!("> ");
        nested::public_function_in_super_mod();
    }

    // `pub(crate)` 使得函数只在当前 crate 中可见
    pub(crate) fn public_function_in_crate() {
        println!("called `my_mod::public_function_in_crate()");
    }

    // 嵌套模块的可见性遵循相同的规则
    mod private_nested {
        #[allow(dead_code)]
        pub fn function() {
            println!("called `my_mod::private_nested::function()`");
        }
    }
}


fn function() {
    println!("called `function()`");
}

mod match_test {
    #[allow(dead_code)]
    pub enum Color {
        // 这三个取值仅由它们的名字（而非类型）来指定。
        Red,
        Blue,
        Green,
        // 这些则把 `u32` 元组赋予不同的名字，以色彩模型命名。
        RGB(u32, u32, u32),
        HSV(u32, u32, u32),
        HSL(u32, u32, u32),
        CMY(u32, u32, u32),
        CMYK(u32, u32, u32, u32),
        RGB2(Option<u32>, u32, u32),
    }

    impl Color {
        pub fn match_2(&self) {
            match self {
                Color::RGB(x, y, z) => println!("Red: {}, green: {}, and blue: {}!", x, y, z),
                _ => println!("other type color"),
            }

        }

        pub fn match_3(&self){
            match self {
                Color::RGB2(Some(n@ 13..= 20), y, z)=>println!("Option bind {}", n),
                Color::RGB2(Some(n), y, z)=>println!("Option bind2 {}", n),
                _ => println!("other type color"),
            }
        }
    }

    pub fn pair_match() {
        let pair = (2, -2);
        println!("tell me {:?}", pair);
        match pair {
            (x, y) if x == y => println!("these twins"),
            (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
            _ => println!("others"),
        }
    }

    fn match_1() {
        let boolean = true;
        // match 也是一个表达式
        let binary = match boolean {
            // match 分支必须覆盖所有可能的值
            false => 0,
            true => 1,
            // 试一试 ^ 将其中一条分支注释掉
        };
    }

    pub fn match_of_reference() {
        let some_var = Color::RGB(1, 2, 3);
        let refer = &4;
        match refer {
            &var => println!("red is {:?}", var),
        }

        match *refer {
            var => println!("red is {:?}", var),
        }

        match refer {
            var => println!("red is {:?}", var),
        }
        let value = 5;
        let mut mut_value = 6;
        mut_value = mut_value + 1;

        match value {
            ref var => println!("red is {:?}", var),
        }
        match mut_value {
            ref mut var => {
                *var = *var + 10;
                println!("red is {:?}", var)
            }
        }
    }
}


fn main() {
    // 模块机制消除了相同名字的项之间的歧义。
    function();
    my_mod::function();

    // 公有项，包括嵌套模块内的，都可以在父模块外部访问。
    my_mod::indirect_access();
    my_mod::nested::function();
    my_mod::call_public_function_in_my_mod();

    // pub(crate) 项可以在同一个 crate 中的任何地方访问
    my_mod::public_function_in_crate();

    // pub(in path) 项只能在指定的模块中访问
    // 报错！函数 `public_function_in_my_mod` 是私有的
    //my_mod::nested::public_function_in_my_mod();
    // 试一试 ^ 取消该行的注释

    // 模块的私有项不能直接访问，即便它是嵌套在公有模块内部的

    // 报错！`private_function` 是私有的
    //my_mod::private_function();
    // 试一试 ^ 取消此行注释

    // 报错！`private_function` 是私有的
    //my_mod::nested::private_function();
    // 试一试 ^ 取消此行的注释

    // Error! `private_nested` is a private module
    //my_mod::private_nested::function();
    // 试一试 ^ 取消此行的注释

    println!("Hello, world!");
    let mut list_a = List::new();
    list_a.prepend(1);
    list_a.prepend(2);
    println!("length of list a is {}", list_a.len());
    println!("string of list is {}", list_a.stringify());


    match_test::pair_match();

    let color = match_test::Color::RGB(12, 16, 53);
    color.match_2();

    let color2 = match_test::Color::RGB2(Some(12), 16, 53);
    color2.match_3();
}
