use std::io;
use crate::factories::GameRuleType;
use crate::player::{Player, PlayerType};
use crate::rule::Rule;

pub struct CliUi ;

impl CliUi {
    pub fn display_welcome(&self) {
        println!("\nWelcome to the classic (or not) game of Rock, Paper, Scissors!")
    }

    pub fn display_players(&self, player_1: &Player, player_2: &Player) {
        println!("\nTwo players play the game:");
        println!("> {}", player_1.get_name());
        println!("> {}", player_2.get_name());
    }

    pub fn display_rules(&self, rules: &Vec<Rule>, score_goal: &u8) {
        println!("\nYou will play using the following rules:");
        for rule in rules.iter() {
            println!("> {}", rule.get_description());
        }
        println!("The game ends when a player reaches {} points.", score_goal);
    }

    pub fn display_scores(&self, player1: &Player, player2: &Player, score_goal: &u8) {
        println!("\nScores");
        println!("> {} has {} points", player1.get_name(), player1.get_score());
        println!("> {} has {} points", player2.get_name(), player2.get_score());
        println!("The game ends when a player reaches {} points.", score_goal);
    }

    pub fn display_winner(&self, player: &Player) {
        println!("\nAnd the winner is {}! Well played!", player.get_name())
    }

    pub fn display_battle(&self, move1: &String, move2: &String) {
        println!("\n>>> {} vs {} <<<", move1, move2)
    }

    pub fn display_equality(&self) {
        println!("\nTie!")
    }

    pub fn display_battle_winner(&self, player: &Player) {
        println!("\n{} wins the round!", player.get_name())
    }

    pub fn display_choice_is_made(&self, player: &Player) {
        println!("\n{} has chosen...", player.get_name())
    }

    pub fn display_choice_invite(&self, possible_values: &Vec<&String>, player: &Player) {
        println!("\n{}, what is your choice ?", player.get_name());
        for (i, value) in possible_values.iter().enumerate() {
            println!("{}> {}", i+1, value)
        }
    }

    pub fn display_message(&self, message: String) {
        println!("\n{}", message)
    }

    pub fn display_invalid_choice(&self) {
        println!("This is not a valid choice...")
    }

    fn display_score_invite(&self) {
        println!("\nScore goal ?")
    }

    fn display_invalid_score(&self) {
        println!("This is not a valid score. Score must be greater than 0...")
    }

    pub fn input_user_choice(&self, possible_values: &Vec<&String>, player: &Player) -> String {
        loop {
            let mut s = String::new();
            self.display_choice_invite(&possible_values, player);
            io::stdin().read_line(&mut s).expect("Failed to read the line.");
            let trimmed = s.trim();
            match trimmed.parse::<u8>() {
                Ok(i) => {
                    let out: usize = (i - 1) as usize;
                    let value = possible_values.get(out);
                    match value {
                        None => self.display_invalid_choice(),
                        Some(s) => {
                            return s.to_string()
                        }
                    }
                },
                Err(..) => self.display_invalid_choice()
            }
        }
    }

    pub fn input_string(&self, message: String) -> String {
        let mut s = String::new();
        self.display_message(message);
        io::stdin().read_line(&mut s).expect("Failed to read the line.");
        s.trim().into()
    }

    pub fn input_score(&self) -> u8 {
        loop {
            let mut s = String::new();
            self.display_score_invite();
            io::stdin().read_line(&mut s).expect("Failed to read the line.");
            let trimmed = s.trim();
            match trimmed.parse::<u8>() {
                Ok(i) => {
                    if i <= 0 {
                        self.display_invalid_score();
                        continue
                    }
                    return i
                },
                Err(..) => self.display_invalid_choice()
            }
        }
    }

    pub fn input_player_type(&self) -> PlayerType {
        loop {
            let mut s = String::new();
            self.display_message("What are you?\n1> AI\n2> Human".into());
            io::stdin().read_line(&mut s).expect("Failed to read the line.");
            let trimmed = s.trim();
            match trimmed.parse::<u8>() {
                Ok(i) => {
                    match i {
                        1 => return PlayerType::AI,
                        2 => return PlayerType::Human,
                        _ => self.display_invalid_choice()
                    }
                },
                Err(..) => self.display_invalid_choice()
            }
        }
    }

    pub fn input_gamerules_type(&self) -> GameRuleType {
        loop {
            let mut s = String::new();
            self.display_message("Choose the rules you want!\n1> Classic\n2> Rock,Paper,Scissors,Lizard,Spock".into());
            io::stdin().read_line(&mut s).expect("Failed to read the line.");
            let trimmed = s.trim();
            match trimmed.parse::<u8>() {
                Ok(i) => {
                    match i {
                        1 => return GameRuleType::Classic,
                        2 => return GameRuleType::TheBigBangTheory,
                        _ => self.display_invalid_choice()
                    }
                },
                Err(..) => self.display_invalid_choice()
            }
        }
    }
}