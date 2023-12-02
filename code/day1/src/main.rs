use std::fs;

fn run(input: String) -> i32 {
    let input: Vec<&str> = input.as_str().split("\n").collect();

    let found_nums: Vec<Vec<&str>> = input.into_iter()
    .map(|text: &str| 
        text.matches(char::is_numeric)
        .collect() )
    .collect();
    //println!("{:?}", found_nums);
    
    let mut numbers : Vec<String> = Vec::new();
    for line in found_nums {
        let mut i = String::from(line[0]);
        i.push_str(line[line.len()-1]);
        numbers.push(i.to_owned())
    }

    let output: i32 = numbers.into_iter()
    .map(|text| 
        text.as_str()
        .parse::<i32>().unwrap())
    .sum();

    return output
}

fn main() {
    let input: String = fs::read_to_string("C:/Users/jackm/rust_projects/adventofcode2023-jm/code/day1/src/inputday1.txt").unwrap();
    let result: i32 = run(input);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_case() {
        let input: String = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet".to_string();
        let check = 142;
        assert_eq!(run(input), check);
    }
}
