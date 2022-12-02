pub fn part_one(input: &str) -> Option<i32> {
    let groups: Vec<Vec<i32>> = input
        .split("\n\n")
        .map(|combo| {
            combo
                .split("\n")
                .filter(|ele| ele.len() > 0)
                .map(|ele| ele.parse().unwrap())
                .collect()
        })
        .collect();

    let max_load = groups.iter().map(|group| group.iter().sum()).max();

    return max_load;
}

pub fn part_two(input: &str) -> Option<i32> {
    let groups: Vec<Vec<i32>> = input
        .split("\n\n")
        .map(|combo| {
            combo
                .split("\n")
                .filter(|ele| ele.len() > 0)
                .map(|ele| ele.parse().unwrap())
                .collect()
        })
        .collect();

    let mut loads: Vec<i32> = groups.iter().map(|group| group.iter().sum()).collect();
    loads.sort();

    let last = loads.len();
    Some(loads[last - 1] + loads[last - 2] + loads[last - 3])
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), None);
    }
}
