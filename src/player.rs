use crate::rule::Rule;
use rand::Rng;
use crate::ui::CliUi;

pub enum PlayerType {
    AI,
    Human
}

type Controller = fn(ruleset: &Vec<Rule>, ui: &CliUi, player: &Player) -> String;

fn controller_ai(ruleset: &Vec<Rule>, ui: &CliUi, player: &Player) -> String {
    let mut possible_values: Vec<&String> = vec![];
    for rule in ruleset.iter() {
        if !possible_values.contains(&rule.get_ability()) {
            possible_values.push(&rule.get_ability())
        }
    }
    let index = rand::thread_rng().gen_range(0..possible_values.len());
    ui.display_choice_is_made(&player);
    String::from(possible_values[index])
}

fn controller_human(ruleset: &Vec<Rule>, ui: &CliUi, player: &Player) -> String {
    let mut possible_values: Vec<&String> = vec![];
    for rule in ruleset.iter() {
        if !possible_values.contains(&rule.get_ability()) {
            possible_values.push(&rule.get_ability())
        }
    }

    ui.input_user_choice(&possible_values, player)
}

pub struct Player {
    name: String,
    score: u8,
    controller: Controller,
}

impl Player {
    pub fn new_human(name: String) -> Self {
        Player { name, score: 0, controller: controller_human }
    }

    pub fn new_ai(name: String) -> Self {
        Player { name, score: 0, controller: controller_ai }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_score(&self) -> &u8 {
        &self.score
    }

    pub fn modify_score(&mut self, amount: u8) {
        self.score += amount
    }

    pub fn next_move(&self, ruleset: &Vec<Rule>, ui: &CliUi) -> String {
        (self.controller)(ruleset, ui, &self)
    }
}