#[cfg(test)]


mod tests {
    use crate::{MD5HashCashInput, MD5HashCashOutput, run_encode};
    use crate::features::{binary_to_hex, check_seed, convert_to_binary_from_hex, decimal_to_binary, get_hash_code, to_seed_value};
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn run_encode_hashcode_test() {
        let input = MD5HashCashInput {
            complexity: 9,
            message: String::from("hello"),
        };
        let output = MD5HashCashOutput{
            seed: 844,
            hashcode: String::from("00441745D9BDF8E5D3C7872AC9DBB2C3"),
        };
        assert_eq!(run_encode(input).hashcode, output.hashcode);

    }

    #[test]
    fn run_encode_seed_test() {
        let input = MD5HashCashInput {
            complexity: 9,
            message: String::from("hello"),
        };
        let output = MD5HashCashOutput{
            seed: 844,
            hashcode: String::from("00441745D9BDF8E5D3C7872AC9DBB2C3"),
        };
        assert_eq!(run_encode(input).seed, output.seed);
    }
    #[test]
    fn check_seed_test_1() {
        let input = MD5HashCashInput {
            complexity: 9,
            message: String::from("hello"),
        };
        let output = MD5HashCashOutput{
            seed: 844,
            hashcode: String::from("00441745D9BDF8E5D3C7872AC9DBB2C3"),
        };
        assert_eq!(check_seed(output.hashcode, input.complexity), true);
    }

    #[test]
    fn run_encode_seed_test_2() {
        let input = MD5HashCashInput {
            complexity: 9,
            message: String::from("hello"),
        };
        let output = MD5HashCashOutput{
            seed: 844,
            hashcode: String::from("00441745D9BDF8E5D3C7872AC9DBB2C3"),
        };
        assert_eq!(check_seed(output.hashcode, 10), false);
    }

    #[test]
    fn to_seed_value_test() {
        let input = MD5HashCashInput {
            complexity: 9,
            message: String::from("hello"),
        };
        let output = MD5HashCashOutput{
            seed: 844,
            hashcode: String::from("00441745D9BDF8E5D3C7872AC9DBB2C3"),
        };
        assert_eq!(to_seed_value(output.seed), "000000000000034C");
    }

    #[test]
    fn get_hash_code_test() {
        let input = MD5HashCashInput {
            complexity: 9,
            message: String::from("hello"),
        };
        let output = MD5HashCashOutput{
            seed: 844,
            hashcode: String::from("00441745D9BDF8E5D3C7872AC9DBB2C3"),
        };
        assert_eq!(get_hash_code(String::from("000000000000034C"), &input.message), output.hashcode);
    }

    #[test]
    fn decimal_to_binary_test() {

        assert_eq!(decimal_to_binary(8), 1000);
    }

    #[test]
    fn binary_to_hex_test() {

        assert_eq!(binary_to_hex(String::from("11111000"), 0), "f8");
    }

    #[test]
    fn convert_to_binary_from_hex_test() {

        assert_eq!(convert_to_binary_from_hex("F8"), "11111000");
    }

}