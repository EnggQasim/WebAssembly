fn main() {
let mut tup = (500, 6.4, 1);
tup.0=200;
let (x, y, z) = tup;
println!("The value of y is: {},{},{}", x,y,z);
}


/*
fn main() {
let a = [10, 20, 30, 40, 50];
for element in a.iter() {
println!("the value is: {}", element);
}
}



fn main() {
let condition = true;
let number = if condition {
5
} else {
6
};
println!("The value of number is: {}", number);
}
*/