#[cfg(test)]
mod tests {
    use ift::eval;

    #[test]
    fn all() {
        assert_eq!(false, eval("GetAllInterfaces | FilterForwardable").is_empty());
        assert_eq!(true, eval("GetAllInterfaces | FilterGlobal").is_empty()); // assuming behind router
        assert_eq!(false, eval("GetAllInterfaces | FilterIPv4 | SortBy \"default\" | FilterFirst").is_empty())
    }
}