use std::error::Error;
use std::fs;

pub fn run(search: Search) -> Result<(), Box<dyn Error>> {
    println!("search target: {}", search.target);
    let contents = fs::read_to_string(search.file)?;
    for line in search_target(&search.target, &contents) {
        println!("result: {}", line)
    }
    Ok(())
}

pub struct Search {
    pub target: String,
    pub file: String,
}

impl Search {
    
    pub fn new(args :&[String]) -> Result<Search, &'static str> {
        if args.len() < 3 {
            return Err("参数错误");
        }
        let target = args[1].clone();
        let file = args[2].clone();

        Ok(Search { target, file })
    }
}

pub fn search_target<'a>(target: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    for line in contents.lines() {
        if line.contains(target) {
            result.push(line)
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_one() {
        let target = "dot";
        let contents = "\
btc a
dot ksm
etc";

        assert_eq!(vec!["dot ksm"], search_target(&target, &contents));
    }
}