use crate::player::Player;
use crate::rule::Rule;
use crate::ui::CliUi;

pub fn run(ui: CliUi, mut player1: Player, mut player2: Player, ruleset: Vec<Rule>, score_goal: u8) {
    ui.display_players(&player1, &player2);
    ui.display_rules(&ruleset, &score_goal);

    let winner = loop {
        let move_player1 = player1.next_move(&ruleset, &ui);
        let move_player2 = player2.next_move(&ruleset, &ui);
        ui.display_battle(&move_player1, &move_player2);
        let result = battle(&ruleset, &move_player1, &move_player2);

        match result {
            BattleResult::Equality => ui.display_equality(),
            BattleResult::Winner(m) => {
                if m.eq(&move_player1) {
                    ui.display_battle_winner(&player1);
                    player1.modify_score(1)
                } else if m.eq(&move_player2) {
                    ui.display_battle_winner(&player2);
                    player2.modify_score(1)
                }
            }
        }

        ui.display_scores(&player1, &player2, &score_goal);
        let winner = get_winner(&player1, &player2, &score_goal);
        if !winner.is_none() {
            break winner.unwrap()
        }
    };
    ui.display_winner(winner);
    ui.input_string("\nGame finished, press Enter to quit...".into());

}

fn get_winner<'a>(player1: &'a Player, player2: &'a Player, score_goal: &u8) -> Option<&'a Player> {
    if player1.get_score() >= score_goal {
        Some(player1)
    } else if player2.get_score() >= score_goal {
        Some(player2)
    } else {
        None
    }
}

enum BattleResult<T> {
    Winner(T),
    Equality,
}

fn battle<'a>(ruleset: &Vec<Rule>, move1: &'a String, move2: &'a String) -> BattleResult<&'a String> {
    for rule in ruleset {
        if rule.check_winner(move1, move2) {
            return BattleResult::Winner(move1);
        }
        if rule.check_winner(move2, move1) {
            return BattleResult::Winner(move2);
        }
    }
    BattleResult::Equality
}