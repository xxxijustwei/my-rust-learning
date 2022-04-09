use std::error::Error;
use std::fs;

pub fn run(profile: Profile) -> Result<(), Box<dyn Error>> {
    println!("profile target: {}", profile.target);
    let contents = fs::read_to_string(profile.file)?;
    for line in search(&profile.target, &contents) {
        println!("result: {}", line)
    }
    Ok(())
}

pub struct Profile {
    pub target: String,
    pub file: String,
}

impl Profile {
    
    pub fn new(mut args: std::env::Args) -> Result<Profile, &'static str> {
        if args.len() < 3 {
            return Err("缺少参数");
        }

        let target = match args.next() {
            Some(value) => value,
            None => return Err("缺少参数 target")
        };
        let file = match args.next() {
            Some(value) => value,
            None => return Err("缺少参数 file")
        };

        Ok(Profile { target, file })
    }
}

pub fn search<'a>(target: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.contains(target))
        .collect()
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

        assert_eq!(vec!["dot ksm"], search(&target, &contents));
    }
}