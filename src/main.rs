use std::ptr::hash;
use std::u64;
use md5;


fn main() {
    println!("Application started");

    let input = MD5HashCashInput {
        complexity: 20,
        message: String::from("hello"),
    };
    println!("{}", encode(input));
}
// md5 execution

fn encode(input: MD5HashCashInput) -> String {
    let complexity = input.complexity;
    let message = input.message;
    let mut comp: u64 = 1;
    let mut is_seed_found = false;
    let mut hash_code: String = String::from("");
    while is_seed_found != true {
        let seed_value = to_seed_value(comp);
        hash_code = get_hash_code(seed_value, &message).to_uppercase();
        is_seed_found = check_seed(hash_code.clone(), complexity);
        comp += 1;
    }
   hash_code

}
fn check_seed(hash_code: String, complexity: u32) -> bool {
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

fn to_seed_value(number: u64) -> String {
    let complexity_bin = decimal_to_binary(number);
    let str = String::from(complexity_bin.to_string());
    String::from(binary_to_hex(str, 8).to_string()).to_uppercase()
}


fn get_hash_code(seed: String, message: &String) -> String {
    let mut seed_value = seed.to_owned();
    let message_value = message.to_owned();
    seed_value.push_str(&message_value);
    let digest = md5::compute(seed_value);
    format!("{:?}", digest)
}

// input

pub struct MD5HashCashInput {
    // complexity in bits
    complexity: u32,
    // message to sign
    message: String,
}

// output

pub struct MD5HashCashOutput {
    // Seed used to solve the challenge
    seed: u64,
    // hashcode found using seed + message
    hashcode: String,
}


/*fn bin_to_decimal(val: String) -> u64 {
    let intval = u64::from_str_radix(&*val, 2).expect("Not a binary number!");
    println!("{}", intval);
    intval
}*/
fn hex_to_decimal(val: String) -> u64  {
    let z = u64::from_str_radix(&*val, 16).expect("Not a binary hex!");
    z
}



fn decimal_to_binary(mut decimal: u64) -> u64 {
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

fn binary_to_hex(val: String, len: usize) -> String {
    let n: u64 = u64::from_str_radix(&*val, 2).unwrap();
    format!("{:01$x}", n, len * 2)
}

fn convert_to_binary_from_hex(hex: &str) -> String {
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
