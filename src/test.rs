pub fn another_one(a: i32) -> i32 {
    let mut b = a;
    b = b + 1;
    let b = b;
    b
}

pub fn tuple_decomposition(tup: (u64, u32, u32)) -> String {
    let (x, y, z) = tup;
    let mut ret = String::from("Decomposed tuple: x - ");
    ret.push_str(&x.to_string());
    ret.push_str(", y - ");
    ret.push_str(&y.to_string());
    ret.push_str(", z - ");
    ret.push_str(&z.to_string());
    ret
}

#[cfg(test)]
mod test_tests {
    use super::*;

    #[test]
    fn decomposing_tuple() {
        assert_eq!("Decomposed tuple: x - 121, y - 0, z - 10", tuple_decomposition((121, 0, 10)));
    }
}