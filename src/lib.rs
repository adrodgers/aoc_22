use std::fs;
use std::error::Error;
use std::path::Path;

pub fn read_txt_to_string(path: &Path)-> Result<String, Box<dyn Error>> {
    let foo: String = fs::read_to_string(path)?;
    Ok(foo.to_string())
}

pub fn parse_calories(input: String) -> Vec<i32> {
    let mut calories: Vec<i32> = Vec::new();
    let mut current_calories_sum: i32 = 0;

    for line in input.lines() {
        if line.is_empty() {
            calories.push(current_calories_sum);
            current_calories_sum = 0;
        } else {
            current_calories_sum += line.parse::<i32>().unwrap()
        }
    }
    calories
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_1() {
        // part 1
        let calories_text = read_txt_to_string(Path::new("/home/anthonydavidrodgers/Documents/rust/advent_of_code_22/input/elf_calories.txt"));
        let mut calories_vec = parse_calories(calories_text.unwrap());
        calories_vec.sort();
        println!("{:?}",calories_vec.last());
        // part 2
        let last_3 = &calories_vec[calories_vec.len()-3..];
        println!("{}", last_3.iter().sum::<i32>())
    }
}
