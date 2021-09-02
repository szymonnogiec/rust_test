use std;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::convert::Infallible;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;
use std::net::IpAddr as IPStd;
use std::path::Iter;
use std::process::exit;
use std::thread;
use std::time::Duration;

use rand::Rng;

// struct User {
//     username: String,
//     email: String,
//     sign_in_count: u64,
//     active: bool,
// }

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn height(&self) -> u32 {
        self.height
    }
    fn double_height(&mut self) {
        self.height = 2 * self.height;
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
    fn name() -> String {
        String::from("Rectangle")
    }
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn rectangle_test() {
    let r1 = Rectangle {
        width: 30,
        height: 20,
    };
    let r2 = Rectangle {
        width: 10,
        height: 20,
    };
    let r3 = Rectangle {
        width: 10,
        height: 10,
    };
    let r4 = Rectangle {
        width: 31,
        height: 10,
    };

    println!("Name: {}", Rectangle::name());
    println!("{:#?}", r1);
    println!("Area is {}", r1.area());
    println!("Height is {}", r1.height());
    println!("{:?} can hold {:?}: {}", r1, r2, r1.can_hold(&r2));
    println!("{:?} can hold {:?}: {}", r1, r3, r1.can_hold(&r3));
    println!("{:?} can hold {:?}: {}", r1, r4, r1.can_hold(&r4));
    println!("{:?}", Rectangle::square(32));
}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum IP {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn ip_test() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    println!("Home : {:?}", home);
    println!("loopback: {:?}", loopback);
    let home2 = IP::V4(127, 0, 0, 1);
    println!("loopback: {:?}", home2);
}

fn opt_test() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let some_val = Some(5);
    if let Some(x) = some_val {
        println!("{}", x);
    }
}

fn vec_test() {
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    let x = &v[1];
    v.push(4);
    println!("{:?}", v);
    let mut e = &mut v[1];
    *e = 32;
    println!("{:?}", v);
    let v2 = vec!["b", "v"];

    let mut vector = vec![1, 2, 3, 4, 5];
    let first = &vector[0];
    vector.push(6);

    for d in &mut vector {
        *d = *d + 1;
        println!("{}", d);
    }

    #[derive(Debug)]
    enum SC {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let mut row = vec![SC::Int(32), SC::Float(3.14), SC::Text(String::from("bla"))];
    println!("{:?}", row);
    row.push(SC::Float(120.4));
    println!("{:?}", row);
}

fn str_test() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world.");
    let s3 = s1 + &s2;
    // println!("{}", s1);
    println!("{}", s2);
    println!("{}", s3);
}

fn chars_test() {
    let hello = String::from("Hello, world");
    for c in hello.chars() {
        println!("{}", c);
    }
}

fn hashmap_test() {
    let mut scores = HashMap::new();
    let blue = String::from("Blue");
    scores.insert(blue, 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let mut init_scores = vec![10, 50];
    let mut scores2: HashMap<_, _> = teams.iter().zip(init_scores.iter()).collect();

    scores.entry(String::from("Blue")).or_insert(32);
    scores.entry(String::from("Red")).or_insert(32);
    println!("{:?}", scores);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for w in text.split_whitespace() {
        let count = map.entry(w).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}

fn ex1() {
    let mut numbers = vec![
        2, 2, 3, 3, 1, 1, 2, 3, 4, 5, 4, 2, 3, 4, 1, 2, 3, 4, 7, 3, 2, 3, 1, 2, 3, 5, 7, 7,
    ];
    numbers.sort();
    let sum: u32 = numbers.iter().sum();
    let v_len = numbers.len() as u32;
    let avg = (sum as f64 / v_len as f64);

    println!("Sum {} vlen {}", sum, v_len);
    println!("Average {}", avg);
    println!("Median {} ", numbers[numbers.len() / 2]);
    let mut hash_map = HashMap::new();
    for v in numbers {
        let count = hash_map.entry(v).or_insert(0);
        *count += 1;
    }
    let mut hash_vec: Vec<(&u32, &i32)> = hash_map.iter().collect();
    hash_vec.sort_by(|a, b| b.1.cmp(a.1));

    println!("Dominata {:?}", hash_vec[0].0);
}

fn panic() {
    let v = vec![1, 2, 3, 4];
    println!("{}", v[99]);
}

fn open_file() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => match File::create("hello.txt") {
            Ok(fc) => fc,
            Err(e) => {
                panic!(
                    "Tried to create a file but there was a problem: {}",
                    e.to_string()
                );
            }
        },
        Err(error) => {
            panic!(
                "There was a problem opening the file: {}",
                error.to_string()
            );
        }
    };
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(err) => return Err(err),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(err) => return Err(err),
    }
}

fn read_username_from_file_with_question() -> Result<String, io::Error> {
    let home: IPStd = "127.0.0.1".parse().unwrap();
    let mut s = String::new();
    let f = File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn struct_template() {
    struct Point<T> {
        x: T,
        y: T,
    }
    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }
    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 2.0 };
}

fn traits_example() {
    pub trait Summary {
        fn summarize_author(&self) -> String;
        fn summarize(&self) -> String {
            format!("Read more from {}...", self.summarize_author())
        }
    }

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewsArticle {
        fn summarize_author(&self) -> String {
            format!("{}", self.author)
        }
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize_author(&self) -> String {
            format!("@{}", self.username)
        }
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }
    fn notify<T: Summary>(item: T) {
        println!("Breaking news! {}", item.summarize());
    }
}

fn conditional_traits() {
    use std::fmt::Display;

    struct Pair<T> {
        x: T,
        y: T,
    }

    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn ttl_test() {
    let string1 = String::from("abcd");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        println!("Longest {}", result);
    }
}

fn ttl_test2() {
    let s: &'static str = "I have a static lifetime";
    struct ImportantExcept<'a> {
        part: &'a str,
    }
    impl<'a> ImportantExcept<'a> {
        fn level(&self) -> i32 {
            3
        }
        fn bla(&self, bla: &str) -> &str {
            println!("bla {}", bla);
            self.part
        }
    }
    let novel = String::from("Bla bla blabson");
    let first_sentence = novel.split('.').next().expect("Couldnt find .");
    let i = ImportantExcept {
        part: first_sentence,
    };
}

fn generic_traits_ttl() {
    use std::fmt::Display;

    fn longest_with_annoucement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where
        T: Display,
    {
        println!("annoucement {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn closure_one() {
    fn generate_workout(intensity: u32, random_number: u32) {
        let mut expensive_result = Cacher::new(|num| {
            println!("calculating slowly...");
            thread::sleep(Duration::from_secs(2));
            intensity
        });

        if intensity < 25 {
            println!("today, do {} pushups!", expensive_result.value(intensity));
            println!("next, do {} situps!", expensive_result.value(intensity));
        } else {
            if random_number == 3 {
                println!("Take a break today! Remember to stay hydrated!");
            } else {
                println!(
                    "Today, run for {} minutes!",
                    expensive_result.value(intensity)
                );
            }
        }
    }
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn iterators() {
    let mut v1 = vec![1, 2, 3];
    for v in v1.iter() {
        println!("{}", v);
    }
    let sum: u32 = v1.iter().sum();
    let asdf: u32 = v1.iter().map(|x| x + 1).sum();
    let blablator: Vec<u32> = v1.iter().map(|x| x + 1).collect();
    let sum: u32 = v1.iter().sum();

    println!("SUM {}", asdf);
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|x| x.size == shoe_size).collect()
}

struct Counter {
    count: u32,
}
impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}
use std::rc::{Rc, Weak};
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::cell::RefCell;
use std::ops::Deref;
use std::sync::{Mutex, MutexGuard};

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

pub trait Messenger {
    fn send(&self, msg: &str);
}
pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}
impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }
    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 0.75 && percentage_of_max <= 0.9 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        } else if percentage_of_max >= 0.9 && percentage_of_max <= 1.0 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        }
    }
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn threads_test() {
    let v = vec![1, 2, 3];
    let join_handle = thread::spawn(move || {
        for i in 1..10 {
            println!("Hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
            println!("Here's the vec: {:?}", v);
        }
    });

    for i in 1..5 {
        println!("Hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    let result = join_handle.join().unwrap();
    println!("Thread result {:?}", result);
}

fn channels_test() {
    use std::sync::mpsc;
    let (tx, rx) = mpsc::channel();
    // let tx1 = mpsc::Sender::clone(&tx);
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("one"),
            String::from("two"),
            String::from("three"),
            String::from("four"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    thread::spawn(move || {
        let vals = vec![
            String::from("five"),
            String::from("six"),
            String::from("seven"),
            String::from("eight"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    for received in rx {
        println!("Got {}", received);
    }
}

fn mutex_test() {
    use std::sync::Mutex;
    use std::sync::Arc;
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = counter.clone();//Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result = {:?}", *counter.lock().unwrap());
}

#[cfg(test)]
mod test {
    use super::*;
    use std::cell::{Ref, RefCell};

    #[test]
    fn call_with_differrent_values() {
        let mut c = Cacher::new(|a| a);

        let v1 = c.value(1);
        let v2 = c.value(2);

        assert_eq!(v2, 1);
    }

    #[test]
    fn finds_in_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneeker"),
            },
            Shoe {
                size: 11,
                style: String::from("chujoczłapy"),
            },
        ];
        let in_my_size = shoes_in_my_size(shoes, 11);
        assert_eq!(
            in_my_size,
            vec![Shoe {
                size: 11,
                style: String::from("chujoczłapy")
            }]
        );
    }

    #[test]
    fn calling_next_directly() {
        let mut counter = Counter::new();
        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }
    #[test]
    fn using_other_iterator_trait_methods() {
        let sum: u32 = Counter::new()
            .zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();
        assert_eq!(sum, 18);
    }
    #[test]
    fn derefs_test() {
        let x = 5;
        let y = MyBox::new(x);
        assert_eq!(5, x);
        assert_eq!(5, *y);
    }
    #[test]
    fn test_drop() {
        let c = CustomSmartPointer {
            data: String::from("My stuff"),
        };
        let d = CustomSmartPointer {
            data: String::from("Other stuff"),
        };
        println!("CustomSmartPointers created");
        use std::mem::drop;
        drop(c);
        println!("CustomSmartPointerc dropped");
    }
    #[test]
    fn test_boxes() {
        let b = Box::new(5);
        println!("{}", b);
        use List::{Cons, Nil};
        let a = Rc::new(Cons(
            Rc::new(RefCell::new(3)),
            Rc::new(Cons(Rc::new(RefCell::new(10)), Rc::new(Nil))),
        ));
        println!("Count after creating a = {}", Rc::strong_count(&a));
        {
            let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
            println!("Count after creating b = {}", Rc::strong_count(&a));
            let c = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
            println!("Count after creating c = {}", Rc::strong_count(&a));
        }
        println!("Count after scope = {}", Rc::strong_count(&a));
    }

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            self.sent_messages.borrow_mut().push(String::from(msg));
        }
    }

    #[test]
    fn sends_an_over_75_perc() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow_mut().len(), 1);
    }

    #[test]
    fn test_tree() {
        let leaf = Rc::new(Node {
            value: 3,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });
        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf)
        );
        {
            let branch = Rc::new(Node {
                value: 5,
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(vec![Rc::clone(&leaf)]),
            });
            *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
            println!(
                "branch strong = {}, weak = {}",
                Rc::strong_count(&branch),
                Rc::weak_count(&branch)
            );
            println!(
                "leaf strong = {}, weak = {}",
                Rc::strong_count(&leaf),
                Rc::weak_count(&leaf)
            );
        }
        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf)
        );
    }
}

fn main() {
    // mutex_test();
    // channels_test();
    // threads_test();
    // boxes();
    // iterators();

    // closure_one();
    // ttl_test2();
    // let number_list = vec![1, 2, 3, 4, 5, 21, 3, 17, 8, 3, 23];
    // let char_list = vec!['a', 'x', 'd'];
    // let result = largest(&number_list);
    // println!("Largest num: {}", result);
    // let result = largest(&char_list);
    // println!("Largest char: {}", result);
    // println!("{:?}", read_username_from_file_with_question());

    // panic();
    // hashmap_test();
    // str_test();
    // ip_test();
    // rectangle_test();
    // let u1 = User {
    //     username: String::from("bla"),
    //     email: String::from("bla@bla.pl"),
    //     sign_in_count: 1,
    //     active: true,
    // };
    // let u2 = User {
    //     username: String::from("bla2"),
    //     ..u1
    // };
    // let mut rng = rand::thread_rng();
    // let number = rng.gen_range(1..101);
    //
    // println!("Guess the number");
    // println!("Input guess");
    // let s = String::from("abc");
    //
    // loop {
    //     let mut guess = String::new();
    //     io::stdin().read_line(&mut guess).expect("Failed to read line");
    //     let guess: u32 = match guess.trim().parse() {
    //         Ok(num) => num,
    //         Err(_) => {
    //             println!("Enter number gamoniu");
    //             continue;
    //         }
    //     };
    //     println!("You guessed {}", guess);
    //
    //     match guess.cmp(&number) {
    //         Ordering::Less => println!("To small"),
    //         Ordering::Greater => println!("To big"),
    //         Ordering::Equal => {
    //             println!("Correct");
    //             break;
    //         }
    //     }
    // }
}
