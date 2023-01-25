extern crate core;

mod rule;
mod factories;
mod player;
mod game;
mod ui;

use crate::factories::{factory_gamerules, factory_player};
use crate::ui::CliUi;

fn run() {
    let ui = CliUi {};
    ui.display_welcome();
    let ruleset = factory_gamerules(&ui);
    let score_goal: u8 = ui.input_score();
    let player_1 = factory_player(&ui, 1);
    let player_2 = factory_player(&ui, 2);

    game::run(ui, player_1, player_2, ruleset, score_goal);
}

fn main() {
    run()
}