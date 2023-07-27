use std::{ fs, path::Path, collections::HashMap };

fn main() {
    let score_map = HashMap::from([
        ("Rock", 1),
        ("Paper", 2),
        ("Scissors", 3),
    ]);

    let win_map = HashMap::from([
        ("Paper", "Rock"),
        ("Rock", "Scissors"),
        ("Scissors", "Paper"),
    ]);

    let opponent_selection = HashMap::from([
        ("A", "Rock"),
        ("B", "Paper"),
        ("C", "Scissors"),
    ]);

    let my_selection = HashMap::from([
        ("X", "Rock"),
        ("Y", "Paper"),
        ("Z", "Scissors"),
    ]);

  //  let path_to_file = Path::new("test-input.txt");
    let path_to_file = Path::new("input.txt");
    let content = fs::read_to_string(path_to_file).unwrap();
    let game_vec: Vec<Vec<&str>> = content
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|row| row.split_whitespace().collect::<Vec<_>>())
        .collect();

    let result = game_vec.into_iter().fold(0, |acc, round| {
        let opponent_choice = opponent_selection.get(round[0]).unwrap();
        let my_choice = my_selection.get(round[1]).unwrap();

        let choice_score = score_map.get(my_choice).unwrap();

        if opponent_choice == my_choice {
            return acc + choice_score + 3;
        } else if win_map.get(my_choice).unwrap() == opponent_choice {
            return acc + choice_score + 6;
        }

        return acc + choice_score.clone();
    });

    let _res: _ = dbg!(result);
}

/*
  1 for Rock
  2 for Paper
  3 for Scissors

  my
  Rock - X
  Paper - Y
  Scissors - Z

  opponent
  Rock - A
  Paper - B
  Scissors - C

  0 - if lost
  3 - draw
  6 - won
*/
