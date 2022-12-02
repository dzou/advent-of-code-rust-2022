#[derive(Debug, PartialEq, Eq)]
enum RpsMove {
    ROCK,
    PAPER,
    SCISSORS,
}

fn get_rps_move(code: &str) -> RpsMove {
    match code {
        "A" | "X" => RpsMove::ROCK,
        "B" | "Y" => RpsMove::PAPER,
        "C" | "Z" => RpsMove::SCISSORS,
        _ => panic!("wtf"),
    }
}

fn get_response(game: (&RpsMove, &str)) -> RpsMove {
    match game {
        (RpsMove::ROCK, "X") => RpsMove::SCISSORS,
        (RpsMove::ROCK, "Y") => RpsMove::ROCK,
        (RpsMove::ROCK, "Z") => RpsMove::PAPER,
        (RpsMove::PAPER, "X") => RpsMove::ROCK,
        (RpsMove::PAPER, "Y") => RpsMove::PAPER,
        (RpsMove::PAPER, "Z") => RpsMove::SCISSORS,
        (RpsMove::SCISSORS, "X") => RpsMove::PAPER,
        (RpsMove::SCISSORS, "Y") => RpsMove::SCISSORS,
        (RpsMove::SCISSORS, "Z") => RpsMove::ROCK,
        (_, _) => panic!(),
    }
}

fn get_points(rpsMove: &RpsMove) -> i32 {
    match rpsMove {
        RpsMove::ROCK => 1,
        RpsMove::PAPER => 2,
        RpsMove::SCISSORS => 3,
    }
}

fn score_game(game: &(RpsMove, RpsMove)) -> i32 {
    let (opp, you) = game;
    let base_score = get_points(&you);

    if opp == you {
        return 3 + base_score;
    } else if (you == &RpsMove::ROCK && opp == &RpsMove::SCISSORS)
        || (you == &RpsMove::SCISSORS && opp == &RpsMove::PAPER)
        || (you == &RpsMove::PAPER && opp == &RpsMove::ROCK)
    {
        return 6 + base_score;
    }

    return base_score;
}

pub fn part_one(input: &str) -> Option<i32> {
    let game_table: Vec<(RpsMove, RpsMove)> = input
        .split("\n")
        .filter(|line| line.len() > 0)
        .map(|row| {
            let game_moves: Vec<&str> = row.split(" ").collect();
            return (get_rps_move(game_moves[0]), get_rps_move(game_moves[1]));
        })
        .collect();

    let score: i32 = game_table.iter().map(|game| score_game(game)).sum();

    return Some(score);
}

pub fn part_two(input: &str) -> Option<i32> {
    let game_table: Vec<(RpsMove, RpsMove)> = input
        .split("\n")
        .filter(|line| line.len() > 0)
        .map(|row| {
            let game_moves: Vec<&str> = row.split(" ").collect();
            let opp_move = get_rps_move(game_moves[0]);
            let my_move = get_response((&opp_move, game_moves[1]));
            return (opp_move, my_move);
        })
        .collect();

    let score: i32 = game_table.iter().map(|game| score_game(game)).sum();

    return Some(score);
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), None);
    }
}
