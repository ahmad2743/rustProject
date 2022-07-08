use serde::{Deserialize, Serialize};
use crate::challenge::Challenge;

#[derive(Deserialize, Serialize, Debug)]
pub struct MD5HashCashChallenge {
    pub input: MD5HashCashInput
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MD5HashCashInput {
    pub complexity: u32,
    pub message: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MD5HashCashOutput {
    seed: u64,
    hashcode: String,
}

impl Challenge for MD5HashCashChallenge {
    type Input = MD5HashCashInput;

    fn name() -> String {
        "MD5HashCash".to_string()
    }

    fn new(input: Self::Input) -> Self {
        MD5HashCashChallenge { input }
    }

    fn solve(input: ) -> Output {
        run_encode(self.input)
    }


}

use std::process::Output;
use crate::models::MD5HashCashInput;
use crate::models::MD5HashCashOutput;


pub fn run_encode(input: MD5HashCashInput) -> MD5HashCashOutput {
    let complexity = input.complexity;
    let message = input.message;
    let mut comp: u64 = 0;
    let mut is_seed_found = false;
    let mut hash_code: String = String::from("");
    let mut seed_value: String = String::from("");
    while is_seed_found != true {
        comp += 1;
        seed_value = to_seed_value(comp);
        hash_code = get_hash_code(seed_value.clone(), &message).to_uppercase();
        is_seed_found = check_seed(hash_code.clone(), complexity);
    }
    MD5HashCashOutput {
        seed: comp,
        hashcode: hash_code,
    }

}
pub fn check_seed(hash_code: String, complexity: u32) -> bool {
    let mut comp = 0;
    let bin  = convert_to_binary_from_hex(&*hash_code);
    for c in bin.chars(){
        if c == '0' {
            comp += 1;
        }
        else {
            break;
        }
    }
    let result = if complexity <= comp { true } else { false };
    result
}

pub fn to_seed_value(number: u64) -> String {
    let complexity_bin = decimal_to_binary(number);
    let str = String::from(complexity_bin.to_string());
    String::from(binary_to_hex(str, 8).to_string()).to_uppercase()
}


pub fn get_hash_code(seed: String, message: &String) -> String {
    let mut seed_value = seed.to_owned();
    let message_value = message.to_owned();
    seed_value.push_str(&message_value);
    let digest = md5::compute(seed_value);
    format!("{:?}", digest).to_uppercase()
}

pub fn decimal_to_binary(mut decimal: u64) -> u64 {
    if decimal == 0 {
        decimal
    } else {
        let mut bits = String::new();

        while decimal > 0 {
            if decimal % 2 == 0 {
                bits.push_str("0");
            } else {
                bits.push_str("1");
            }

            decimal /= 2;
        }

        // reverse the bits
        match bits.chars().rev().collect::<String>().parse() {
            Ok(num) => num,
            Err(_e) => panic!("Something went wrong"),
        }
    }
}

pub fn binary_to_hex(val: String, len: usize) -> String {
    let n: u64 = u64::from_str_radix(&*val, 2).unwrap();
    format!("{:01$x}", n, len * 2)
}

pub fn convert_to_binary_from_hex(hex: &str) -> String {
    hex[0..].chars().map(to_binary).collect()
}

fn to_binary(c: char) -> &'static str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => "",
    }
}


