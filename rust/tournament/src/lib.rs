use std::collections::HashMap;

const HEADER: &str = "Team                           | MP |  W |  D |  L |  P\n";

const TEAM_NAMES: [&str; 4] = [
    "Devastating Donkeys",
    "Allegoric Alaskans",
    "Blithering Badgers",
    "Courageous Californians",
];

enum Outcome {
    Win = 3,
    Loss = 0,
    Draw = 1,
}

#[derive(Default, Debug)]
struct Team {
    matches: u16,
    wins: u16,
    draws: u16,
    losses: u16,
    points: u16,
}

impl Team {
    pub fn new() -> Self {
        Team {
            ..Default::default()
        }
    }
}

fn update_score(all_teams: &mut HashMap<String, Team>, team: &str, outcome: Outcome) {
    let team_score = all_teams.get_mut(&(team.to_string())).unwrap();
    match outcome {
        Outcome::Win => {
            team_score.wins += 1;
        }
        Outcome::Loss => {
            team_score.losses += 1;
        }
        Outcome::Draw => {
            team_score.draws += 1;
        }
    }
    team_score.points += outcome as u16;
    team_score.matches += 1;
}

pub fn tally(match_results: &str) -> String {
    let mut result = String::from(HEADER);

    let mut all_teams = TEAM_NAMES
        .iter()
        .map(|v| (v.to_string(), Team::new()))
        .collect::<HashMap<String, Team>>();

    for line in match_results.split('\n') {
        let mut iter = line.split(';');
        let first_team = iter.next().unwrap();
        if first_team == "" {
            return result.trim_end_matches('\n').to_string();
        }
        let second_team = iter.next().unwrap();
        if let Some(outcome) = iter.next() {
            match outcome {
                "win" => {
                    update_score(&mut all_teams, &first_team, Outcome::Win);
                    update_score(&mut all_teams, &second_team, Outcome::Loss);
                }
                "loss" => {
                    update_score(&mut all_teams, &first_team, Outcome::Loss);
                    update_score(&mut all_teams, &second_team, Outcome::Win);
                }
                "draw" => {
                    update_score(&mut all_teams, &first_team, Outcome::Draw);
                    update_score(&mut all_teams, &second_team, Outcome::Draw);
                }
                _ => panic!("{} not a valid outcome for a match", outcome),
            }
        }
    }
    let mut count_vec: Vec<(&String, &Team)> = all_teams.iter().collect();
    count_vec.sort_unstable_by_key(|&x| x.0); // ties are settled by teams alphabetical name
    count_vec.sort_by(|&a, &b| b.1.points.cmp(&a.1.points)); // sorted descending order by points

    for (team_name, team) in count_vec.iter().filter(|x| x.1.matches > 0) {
        result.push_str(
            format!(
                "{:30} | {:2} | {:2} | {:2} | {:2} | {:2}\n",
                team_name, team.matches, team.wins, team.draws, team.losses, team.points
            )
            .as_str(),
        )
    }
    result.trim_end_matches('\n').to_string()
}
