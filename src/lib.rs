pub fn doit() -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 0;
        assert_eq!(doit(), result);
    }
}
