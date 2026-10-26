use std::cell::RefCell;
use std::rc::Rc;

use smart_pointers::{CustomSmartPointer, List, MyBox, hello};

fn main() {
    // 1. 解引用与解引用强制转换
    let x = 5;
    let y = MyBox::new(x);
    println!("x = {x}, *y = {}", *y);
    println!("{}", hello(&MyBox::new(String::from("Rust"))));

    // 2. Drop：注意打印顺序——作用域结束的瞬间自动清理
    {
        let _c = CustomSmartPointer {
            name: String::from("inside inner scope"),
        };
        println!("CustomSmartPointer created inside scope");
    }
    println!("after inner scope");

    // 3. Rc 共享所有权：三个列表共享同一条尾巴
    let a = Rc::new(List::Cons(5, Rc::new(List::Cons(21, Rc::new(List::Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));

    let b = List::Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));

    {
        let c = List::Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
        drop(c); // 也可以提前手动交还
        println!("count after dropping c = {}", Rc::strong_count(&a));
    }

    println!("b = {b:?}");

    // 4. RefCell：不可变值内部的"合法"修改
    let shared = Rc::new(RefCell::new(vec![1, 2, 3]));
    let another = Rc::clone(&shared);
    another.borrow_mut().push(4); // another 没有 mut 也能改内部数据
    println!("shared = {:?}", shared.borrow());
}
