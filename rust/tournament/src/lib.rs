use std::collections::HashMap;

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
    name: String,
    matches: u16,
    wins: u16,
    draws: u16,
    losses: u16,
    points: u16,
}

impl Team {
    pub fn new(name: String) -> Self {
        Team {
            name,
            ..Default::default()
        }
    }
}

impl PartialEq for Team {
    fn eq(&self, rhs: &Team) -> bool {
        self.name == rhs.name
    }
}

impl PartialOrd for Team {
    fn partial_cmp(&self, rhs: &Team) -> Option<std::cmp::Ordering> {
        Some(self.cmp(rhs))
    }
}

impl Eq for Team {}

impl Ord for Team {
    fn cmp(&self, other: &Team) -> std::cmp::Ordering {
        if self.points == other.points {
            self.name.cmp(&other.name)
        } else {
            // since it is decreasing order
            other.points.cmp(&self.points)
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
    let mut all_teams = TEAM_NAMES
        .iter()
        .map(|v| (v.to_string(), Team::new(v.to_string())))
        .collect::<HashMap<String, Team>>();
    for line in match_results.split('\n') {
        let mut iter = line.split(';');
        let first_team = iter.next().unwrap();
        if first_team == "" {
            break;
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
    let mut result = format!("{:30} | MP |  W |  D |  L |  P\n", "Team");
    if all_teams.is_empty() {
        return result.trim_end_matches('\n').to_string();
    }
    let mut count_vec: Vec<&Team> = all_teams.iter().map(|(_, y)| y).collect();
    count_vec.sort();
    for team in count_vec.iter().filter(|x| x.matches > 0) {
        result.push_str(
            format!(
                "{:30} | {:2} | {:2} | {:2} | {:2} | {:2}\n",
                team.name, team.matches, team.wins, team.draws, team.losses, team.points
            )
            .as_str(),
        )
    }
    result.trim_end_matches('\n').to_string()
}
