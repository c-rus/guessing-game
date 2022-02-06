/// Adds 1 to `num`.  
pub fn increment(num: u32) -> u32 {
    return num + 1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn increments_by_1() {
        assert_eq!(increment(20), 21);

        let n1: u32 = 0;
        assert_eq!(increment(n1), 0 + 1);
    }
}
