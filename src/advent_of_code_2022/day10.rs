fn day10(input: &String) -> i32 {
    let lines = input.split("\n").collect::<Vec<&str>>();
    let mut amounts = vec![1];
    for l in lines {
        if l.starts_with("noop") {
            amounts.push(0);
        } else if l.starts_with("addx") {
            amounts.push(0);
            let to_add = (l.split(" ").collect::<Vec<&str>>()[1]).to_string().parse::<i32>().unwrap();
            amounts.push(to_add);
        }
    }

    let mut total: i32 = 0;
    for i in [20, 60, 100, 140, 180, 220] {
        total += (i as i32) * amounts[0..i].iter().sum::<i32>();
    }
    total
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn run_with_real_input() {
        // load from file...
        let file_path = "/home/emmasmith/Development/scratch_python/advent_of_code_2022/input10.txt";
        let contents = fs::read_to_string(file_path)
            .expect("Should have been able to read the file");
        assert_eq!(day10(&contents), 14040)
    }

    #[test]
    fn run_with_test_input() {
        let test_input = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop".to_string();
        assert_eq!(day10(&test_input), 13140)
    }
}
