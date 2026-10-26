use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

// ── Box<T>：堆分配，最简单的智能指针 ────────────────────────────
//
// 这里不是用标准库的 Box，而是手写一个 MyBox 来看清智能指针的本质：
// 它就是一个包着值的结构体，实现 Deref 后就能表现得像它包的东西。

pub struct MyBox<T>(T);

impl<T> MyBox<T> {
    pub fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// 实现 Deref 才支持"解引用强制转换"（deref coercion）：
// &MyBox<String> 会自动一层层转成 &String 再转成 &str，
// 所以函数要 &str 时，传 &MyBox<String> 也能编译通过。
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

pub fn hello(name: &str) -> String {
    format!("Hello, {name}!")
}

// ── Drop：离开作用域时自动执行的清理逻辑 ────────────────────────

pub struct CustomSmartPointer {
    pub name: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.name);
    }
}

// ── Rc<T>：引用计数的共享所有权 ────────────────────────────────
//
// 普通所有权一个值只能有一个所有者；Rc 允许多个所有者共享同一个值，
// 最后一个所有者离开作用域时才真正清理。

#[derive(Debug)]
pub enum List {
    Cons(i32, Rc<List>),
    Nil,
}

// ── RefCell<T>：内部可变性（把借用检查推迟到运行时）────────────
//
// 编译期的借用规则（多个 & 或一个 &mut）在 RefCell 里变成运行时规则：
// 运行时同时存在两个可借用者会 panic。好处是"不可变值内部改数据"成为可能。

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mybox_derefs_to_inner_value() {
        let x = 5;
        let y = MyBox::new(x);
        assert_eq!(5, x);
        assert_eq!(5, *y); // * 号触发 Deref::deref
    }

    #[test]
    fn deref_coercion_passes_mybox_where_str_is_expected() {
        let m = MyBox::new(String::from("Rust"));
        assert_eq!(hello(&m), "Hello, Rust!"); // &MyBox<String> → &str 自动转换
    }

    #[test]
    fn rc_counts_owners_and_cleans_up_at_zero() {
        let a = Rc::new(List::Cons(5, Rc::new(List::Cons(21, Rc::new(List::Nil)))));
        assert_eq!(Rc::strong_count(&a), 1);

        let _b = List::Cons(3, Rc::clone(&a)); // clone 只增加计数，不复制数据
        assert_eq!(Rc::strong_count(&a), 2);

        {
            let _c = List::Cons(4, Rc::clone(&a));
            assert_eq!(Rc::strong_count(&a), 3);
        } // _c 离开作用域，计数减一

        assert_eq!(Rc::strong_count(&a), 2);
    }

    #[test]
    fn refcell_allows_sequential_mutable_borrows() {
        let value = RefCell::new(5);

        *value.borrow_mut() += 10;
        assert_eq!(*value.borrow(), 15);

        *value.borrow_mut() += 5;
        assert_eq!(*value.borrow(), 20);
    }

    #[test]
    #[should_panic(expected = "already borrowed")]
    fn refcell_panics_on_overlapping_borrows() {
        let value = RefCell::new(5);

        let _immutable = value.borrow();
        let _mutable = value.borrow_mut(); // 运行时 panic：不可变借用还活着
    }
}
