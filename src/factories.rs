use crate::rule;
use rule::Rule;
use crate::player::{Player, PlayerType};
use crate::ui::CliUi;

pub enum GameRuleType {
    Classic,
    TheBigBangTheory
}

pub fn classic_ruleset() -> Vec<Rule> {
    vec!(
        Rule::new("Rock".into(), "Scissors".into(), "Rock smashes scissors".into()),
        Rule::new("Paper".into(), "Rock".into(), "Paper covers rock".into()),
        Rule::new("Scissors".into(), "Paper".into(), "Scissors cuts paper".into())
    )
}

pub fn tbbt_ruleset() -> Vec<Rule> {
    vec!(
        Rule::new("Scissors".into(), "Paper".into(), "Scissors cuts Paper".into()),
        Rule::new("Paper".into(), "Rock".into(), "Paper covers Rock".into()),
        Rule::new("Rock".into(), "Lizard".into(), "Rock crushes Lizard".into()),
        Rule::new("Lizard".into(), "Spock".into(), "Lizard poisons Spock".into()),
        Rule::new("Spock".into(), "Scissors".into(), "Spock smashes Scissors".into()),
        Rule::new("Scissors".into(), "Lizard".into(), "Scissors decapitates Lizard".into()),
        Rule::new("Lizard".into(), "Paper".into(), "Lizard eats Paper".into()),
        Rule::new("Paper".into(), "Spock".into(), "Paper disproves Spock".into()),
        Rule::new("Spock".into(), "Rock".into(), "Spock vaporizes Rock".into()),
        Rule::new("Rock".into(), "Scissors".into(), "(and as it always has) Rock crushes Scissors".into()),
    )
}

pub fn factory_player(ui: &CliUi, player_number: u8) -> Player {
    let player_name = ui.input_string(format!("Player {}, what's your name?", player_number));
    let player_type = ui.input_player_type();

    match player_type {
        PlayerType::AI => Player::new_ai(player_name),
        PlayerType::Human => Player::new_human(player_name)
    }
}

pub fn factory_gamerules(ui: &CliUi) -> Vec<Rule> {
    match ui.input_gamerules_type() {
        GameRuleType::Classic => classic_ruleset(),
        GameRuleType::TheBigBangTheory => tbbt_ruleset()
    }
}