//
// // Obtain the number of bytes (not characters) in the given argument
// // Add the AsRef trait appropriately as a trait bound
// fn byte_counter<T: AsRef<str>>(arg: T) -> usize {
//     arg.as_ref().as_bytes().len()
// }
//
// // Obtain the number of characters (not bytes) in the given argument
// // Add the AsRef trait appropriately as a trait bound
// fn char_counter<T: AsRef<str>>(arg: T) -> usize {
//     arg.as_ref().chars().count()
// }

// how to solve this? Could explore all possible options. Greedy algorithm won't work
// Exploring everything is the best I can think of at the moment

use std::collections::HashMap;
use regex::Regex;

fn day16(input: &String) -> u32 {
    let lines = input.split("\n").collect::<Vec<&str>>();
    let mut adjacency = HashMap::new();
    let mut flows = HashMap::new();
    let re = Regex::new(r"^Valve ([A-Z]{2}) has flow rate=([0-9]+); tunnels lead to valves (.*$)").unwrap();
    for line in lines {
        match re.captures(line) {
            Some(cap) => {
                flows.insert(cap[1].to_string(), cap[2].to_string());
                let tun = cap[3].split(", ").map(|a| a.to_string()).collect::<Vec<_>>();
                adjacency.insert(cap[1].to_string(), tun);
                // println!("tunnels to {}", &cap[3]);
            },
            None => continue
        }
    }
    // finally, now to do the exhaustive search
    let start = "AA";



    input.len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn different_counts() {
        let test_input = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";
        assert_eq!(day16(&test_input.to_string()), 561);
    }
}
