// fn main() {
// // let mut s = String::from("hello");
// // s.push_str(", world!"); // push_str() appends a literal to a String
// // println!("{}", s); // This will print `hello, world!`

// // ====4.2
// // let x = 5;
// // let y = x;
// // println!("{}, {}",x,y);


// // ====4.3
// // let s1 = String::from("hello");
// // let s2 = s1;
// // println!("{}",s2);


// //======4.5
// // let s1 = String::from("hello");
// // let s2 = s1.clone();
// // println!("s1 = {}, s2 = {}", s1, s2);


// //=========4.7
// let s = String::from("hello");
// //takes_ownership(s);

// let x = 5;
// makes_copy(x);

// println!("{},",x);
// }

// // 4.7 start
// fn takes_ownership(some_string: String) { // some_stringcomes into scope.
// println!("{}", some_string);
// }

// fn makes_copy(some_integer: i32) { // some_integer comes into scope.
// println!("{}", some_integer);
// } // Here, some_inlet x = 5;
// //teger goes out of scope. Nothing special happens.
// // end 4.7


//==============Ref and borrowing
// fn main() {
// let s1 = String::from("hello");
// let (s2, len) = calculate_length(s1);
// println!("The length of '{}' is {}.", s2, len);
// }
// fn calculate_length(s: String) -> (String, usize) {
// let length = s.len(); // len() returns the length of a String.
// (s, length)
// }

// fn main() {
// let s1 = String::from("hello");
// let len = calculate_length(&s1);
// println!("The length of '{}' is {}.", s1, len);
// }
// fn calculate_length(s: &String) -> usize {
//     s.len()
// }


//===========>Mutable References
// fn main() {
// let mut s = String::from("hello");
// change(&mut s);
// println!("{},",s)
// }
// fn change(some_string: &mut String) {
// some_string.push_str(", world");
// }

//==========>Only one mutable reference allow
fn main(){
let mut s = "hello";
let r2=s.push_str("123");
println!("{},{}", s,r2);
}
//let r2 = &mut s;


// //============>slicing
// fn main(){
//     let s = String::from("hello");
// println!("{}", s);
// let slice = &s[0..2];
// println!("{}", slice);
// let slice = &s[..2];
// println!("{}", slice);
// let slice = &s[..];
// println!("{}", slice);
// }