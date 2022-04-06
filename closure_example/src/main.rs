use closure_example::BetterCacher;


fn main() {
    let mut cacher = BetterCacher::new(|args| {
        args + 1
    });

    let result_a = cacher.execute(10);
    let result_b = cacher.execute(20);

    for ele in cacher.result {
        println!("{}-{}", ele.0, ele.1)
    }

    assert_eq!(11, result_a);
    assert_eq!(21, result_b);
}

#[cfg(test)]
mod test {
    use closure_example::{Cacher, BetterCacher};

    
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
}
