use ::lib::grid::Action;
use ::lib::creature::*;

pub fn new() -> Creature {
    Creature {
        creature_type: CreatureType::Wall,
        color: [1.0, 1.0, 1.0, 1.0],
        energy: 0,
        sleep: 0,
        properties: HashMap::new(),
        action: wall_action,
    }
}

#[allow(unused_variables)]
fn wall_action(myself: &mut Creature, neighbors: &Neighbors) -> Vec<Action> {
    vec![Action::Idle]
}