use std::fmt::Display;
use std::ops::Deref;
use std::rc::{Rc, Weak};
use std::cell::{RefCell};
use crate::smart_pointer::List::{Cons, Nil};

struct TMBox<T>(T) where T: Display;

impl<T> TMBox<T>
    where T: Display {
    fn new(x: T) -> TMBox<T> {
        TMBox(x)
    }
}

impl<T> Deref for TMBox<T>
    where T: Display {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Drop for TMBox<T>
    where T: Display {
    fn drop(&mut self) {
        println!("Now drop the value. {}", self.0);
    }
}

/// 순환 참조 관련 테스트를 위한 List 구조체입니다.
///
/// ~~~ rust
/// use std::rc::{Rc, Weak};
/// use std::cell::RefCell;
///
/// let leaf = Rc::new(Node {
///     value: 1,
///     parent: RefCell::new(Weak::new()),
///     children: RefCell::new(vec![])
/// });
///
/// println!("Leaf strong ref: {}, weak ref: {}",
/// Rc::strong_count(&leaf),
/// Rc::weak_count(&leaf)
/// );
///
/// {
///     let branch = Rc::new( Node {
///         value: 2,
///         parent: RefCell::new(Weak::new()),
///         children: RefCell::new(vec![Rc::clone(&leaf)]),
///     });
///
///     *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
///
///     println!("Branch strong ref: {}, weak ref: {}",
///         Rc::strong_count(&branch),
///         Rc::weak_count(&branch)
///     );
///     println!("Leaf strong ref: {}, weak ref: {}",
///         Rc::strong_count(&leaf),
///         Rc::weak_count(&leaf)
///     );
/// }
///
/// println!("Leaf parent: {:?}", leaf.parent.borrow().upgrade());
/// println!("Leaf strong ref: {}, weak ref: {}",
///     Rc::strong_count(&leaf),
///     Rc::weak_count(&leaf)
/// );
/// ~~~
#[derive(Debug)]
pub enum List<T> {
    Cons(T, RefCell<Rc<List<T>>>),
    Nil,
}

impl<T> List<T> {
    pub fn tail(&self) -> Option<&RefCell<Rc<List<T>>>> {
        match *self {
            Cons(_, ref item) => Some(item),
            Nil => None,
        }
    }
}

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T>
    where T: 'a + Messenger {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
    where T: Messenger {
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 0.75 && percentage_of_max < 0.9 {
            self.messenger.send("Warning: You've used up over 75% of your quota!");

        } else if percentage_of_max >= 0.9 && percentage_of_max < 1.0 {
            self.messenger.send("Urgent warning: You've used up over 90% of your quota!");

        } else if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        }
    }
}

struct MockMessenger {
    send_messages: RefCell<Vec<String>>,
}

impl MockMessenger {
    fn new() -> MockMessenger {
        MockMessenger { send_messages: RefCell::new(vec![]) }
    }
}

impl Messenger for MockMessenger {
    fn send(&self, msg: &str) {
        self.send_messages.borrow_mut().push(String::from(msg));
    }
}

#[derive(Debug)]
struct Node<T> {
    value: T,
    parent: RefCell<Weak<Node<T>>>,
    children: RefCell<Vec<Rc<Node<T>>>>,
}