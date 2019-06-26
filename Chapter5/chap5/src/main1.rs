// fn main() {
//     // struct User {
//     // username: String,
//     // email: String,
//     // sign_in_count: u64,
//     // active: bool,
//     // }


//     // let user1 = User {
//     // email: String::from("someone@example.com"),
//     // username: String::from("someusername123"),
//     // active: true,
//     // sign_in_count: 1,
//     // };

//     // println!("{}",user1.email);


//     //=========Listing 5.3
//     // fn build_user(email: String, username: String) -> User
//     // {
//     //     User {
//     //     email: email,
//     //     username: username,
//     //     active: true,
//     //     sign_in_count: 1,
//     //     }
//     // }


//     //=============Listing 5.4

//     // fn build_user(email: String, username: String) -> User
//     // {
//     //     User {
//     //         email,
//     //         username,
//     //         active: true,
//     //         sign_in_count: 1,
//     //     }
//     // }

// //==================Listing 5-5:
// // let user2 = User {
// // email: String::from("another@example.com"),
// // username: String::from("anotherusername567"),
// // active: user1.active,
// // sign_in_count: user1.sign_in_count,
// // };

// //===================Listing 5-6:

//     struct User {
//     username: String,
//     email: String,
//     sign_in_count: u64,
//     active: bool,
//     }
//     let user1 = User {
//     email: String::from("someone@example.com"),
//     username: String::from("someusername123"),
//     active: true,
//     sign_in_count: 1,
//     };

//     let user2 = User {
//     email: String::from("another@example.com"),
//     username: String::from("anotherusername567"),
//     ..user1

//     };

// }
//========================Listing 5-11
// #[derive(Debug)]
// struct Rectangle {
// length: u32,
// width: u32,
// }
// fn main() {
// let rect1 = Rectangle { length: 50, width: 30 };
// println!("rect1 is {:#?}", rect1);
// }