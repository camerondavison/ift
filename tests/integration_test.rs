#[cfg(test)]
mod tests {
    use ift::{eval, evals};

    #[test]
    fn all() {
        assert_eq!(true, evals("GetAllInterfaces | FilterForwardable").is_some());
        assert_eq!(false, evals("GetAllInterfaces | FilterGlobal").is_some()); // assuming behind router
        assert_eq!(
            true,
            evals("GetAllInterfaces | FilterIPv4 | SortBy \"default\" | FilterFirst").is_some()
        )
    }

    #[test]
    fn it_fails() {
        eval("adoe").expect_err("should fail");
    }
}
