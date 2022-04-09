fn main() {

}

#[cfg(test)]
mod test {

    use iterator_example::*;

    #[test]
    fn test_one() {
        let cart = vec![
            Clothes::new(String::from("MYGE"), 2021, Season::SS, String::from("人类思维序列概念重磅T恤"), 299.0),
            Clothes::new(String::from("USAGE"), 2022, Season::SS, String::from("长绒棉廓形套头卫衣"), 369.0),
            Clothes::new(String::from("FREAK'S STORE"), 2022, Season::SS, String::from("刺绣维尼熊印花T恤"), 378.0)
        ];

        let buy: Vec<_> = cart.into_iter().filter(|clothes| clothes.year == 2022).collect();

        assert_eq!(
            vec![
                    Clothes::new(String::from("USAGE"), 2022, Season::SS, String::from("长绒棉廓形套头卫衣"), 369.0),
                    Clothes::new(String::from("FREAK'S STORE"), 2022, Season::SS, String::from("刺绣维尼熊印花T恤"), 378.0)
                ],
            buy
        )
    }

    #[test]
    fn test_two() {
        let result = Counter::new()
            .zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|value| value % 3 == 0)
            .sum::<u32>();

        assert_eq!(18, result);
    }
}