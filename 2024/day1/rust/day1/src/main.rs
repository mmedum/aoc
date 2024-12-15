use std::collections::{HashMap, HashSet};

fn main() {
    let input: &str = include_str!("../input");
    println!("Part1: {}", part1(input));
    println!("Part2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let pairs = input.lines().map(|line| {
        let (i, j) = line.split_once("   ").unwrap();
        let i: usize = i.parse().unwrap();
        let j: usize = j.parse().unwrap();

        (i, j)
    });

    let (mut lefts, mut rights): (Vec<_>, Vec<_>) = pairs.unzip();

    lefts.sort();
    rights.sort();

    let mut result = 0;
    for (lval, rval) in lefts.into_iter().zip(rights.into_iter()) {
        result += lval.abs_diff(rval);
    }

    return result;
}

fn part2(input: &str) -> usize {
    let (mut keys, mut value_counts): (HashSet<usize>, HashMap<usize, usize>) =
        (HashSet::new(), HashMap::new());

    for line in input.lines() {
        let (i, j) = line.split_once("   ").unwrap();
        let i: usize = i.parse().unwrap();
        let j: usize = j.parse().unwrap();

        keys.insert(i);
        value_counts.entry(j).and_modify(|v| *v += 1).or_insert(1);
    }

    let mut result = 0;
    for key in keys {
        result += key * value_counts.get(&key).unwrap_or(&0)
    }

    return result;
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("../input")), 1506483);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("../input")), 23126924);
    }
}
