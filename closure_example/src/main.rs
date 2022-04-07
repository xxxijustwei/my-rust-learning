use closure_example::BetterCacher;


fn main() {
    let mut cacher = BetterCacher::new(|args| {
        args + 1
    });

    let result_a = cacher.execute(10);
    let result_b = cacher.execute(20);

    for ele in cacher.results {
        println!("{}-{}", ele.0, ele.1)
    }

    assert_eq!(11, result_a);
    assert_eq!(21, result_b);
}

#[cfg(test)]
mod test {
    use closure_example::{Cacher, BetterCacher, SuperCacher};

    
    #[test]
    #[should_panic]
    fn test_one() {
        let mut cacher = Cacher::new(|args| {
            args + 1
        });

        let result_a = cacher.execute(10);
        let result_b = cacher.execute(20);

        assert_eq!(11, result_a);
        assert_eq!(21, result_b);
    }

    #[test]
    fn test_two() {
        let mut cacher = BetterCacher::new(|args| {
            args + 1
        });

        let result_a = cacher.execute(10);
        let result_b = cacher.execute(20);

        assert_eq!(11, result_a);
        assert_eq!(21, result_b);
    }

    #[test]
    fn test_three() {
        let mut int_cacher = SuperCacher::new(|args| {
            args + 10
        });

        let result_a = int_cacher.execute(8);

        let mut len_cacher = SuperCacher::new(|args: &str| {
            let mut value = args.to_string();
            value.push_str(".");
            value
        });

        let result_b = len_cacher.execute("my name is justwei");

        assert_eq!(18, result_a);
        assert_eq!("my name is justwei.", result_b);
    }
}
