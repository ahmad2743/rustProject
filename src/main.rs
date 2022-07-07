mod features;
mod models;

use md5;
use crate::features::{run_encode};
use crate::models::MD5HashCashInput;
use crate::models::MD5HashCashOutput;



fn main() {
    println!("Application started");

    let input = MD5HashCashInput {
        complexity: 9,
        message: String::from("hello"),
    };
    println!("{:?}", run_encode(input));
}
// md5 execution


