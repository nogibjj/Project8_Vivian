#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn test_program_success() {
        let result = run_program().unwrap();
        assert_eq!(result, 1);
    }
}

