pub struct Rule {
    ability: String,
    opponent: String,
    description: String,
}

impl Rule {
    pub fn new(ability: String, opponent: String, description: String) -> Rule {
        Rule { ability, opponent, description }
    }

    pub fn get_ability(&self) -> &String {
        &self.ability
    }

    pub fn get_description(&self) -> &String {
        &self.description
    }

    pub fn check_winner(&self, ability: &String, opponent: &String) -> bool {
        // println!("{} vs {}", ability, opponent);
        self.ability.eq(ability) && self.opponent.eq(opponent)
    }
}