extern crate rand;

use rand::Rng;

mod test;

fn main() {

    println!("Hello, world! {} {}", testing_function(5), rand::thread_rng().gen_range(1, 101));
    println!("{}", test::another_one(0));

    let tup = (131u64, 0u32, 12u32);
    println!("Decomposing tuple {:?}", tup);
    println!("{}", test::tuple_decomposition(tup));
}

fn testing_function(input: i32) -> i32 {

    input
}

#[cfg(test)]
mod main_tests {
    use super::*;

    #[test]
    fn testing_function_send_5_output_5() {
        assert_eq!(5, testing_function(5));
    }

    #[test]
    fn testing_function_send_0_output_0() {
        assert_eq!(0, testing_function(2 - 2));
    }
}

