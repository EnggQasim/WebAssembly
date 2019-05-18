fn main() {
let x = plus_one(5);
println!("The value of x is: {}", x);
}
fn plus_one(x: i32) -> i32 {
x + 1 // not use semicolon in the end where return value
}

/*
fn five() -> i32 {
5
}
fn main() {
let x = five();
println!("The value of x is: {}", x);
}



fn main() {
another_function(5, 6);
}
fn another_function(x: i32, y: i32) {
println!("The value of x is: {}", x);
println!("The value of y is: {}", y);
}


fn main() {
another_function(5);
}
fn another_function(x: i32) {
println!("The value of x is: {}", x);
}


fn main() {
println!("Hello, world!");
another_function();
}

fn another_function() {
println!("Another function.");
}*/