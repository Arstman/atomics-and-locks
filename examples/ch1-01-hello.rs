use std::thread;

fn main() {
    thread::spawn(f);
    thread::spawn(f);

    println!("Hello, from main thread!");
}

fn f() {
    println!("Hello, from spawned thread!");

    let id = thread::current().id();
    println!("This is my thread id: {id:?}");
}
