mod a_star;

use a_star::{a_star, Neighbor};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Pos {
    Sink,
    Counter,
    Stove,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Holdable {
    Pot,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Activatable {
    Stove,
    Faucet,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct State {
    pub pos: Pos,
    pub pot_pos: Pos,
    pub pot_filled: bool,
    pub faucet_on: bool,
    pub stove_on: bool,
    pub holding: Option<Holdable>,
}

fn heuristic(state: &State) -> f32 {
    let mut distance = 0.0;

    if state.pot_pos != Pos::Stove {
        distance += 1.0;
    }

    if !state.stove_on {
        distance += 1.0;
    }

    if !state.pot_filled {
        distance += 1.0;
    }

    if state.holding != None {
        distance += 1.0;
    }

    if state.faucet_on {
        distance += 1.0;
    }

    distance
}

fn neighbors(state: &State) -> Vec<Neighbor<State>> {
    let mut neighbors = vec![];
    neighbors.push(move_actor(state, Pos::Sink));
    neighbors.push(move_actor(state, Pos::Counter));
    neighbors.push(move_actor(state, Pos::Stove));
    neighbors.push(pick_up(state, Holdable::Pot));
    neighbors.push(put_down(state));
    neighbors.push(turn_on(state, Activatable::Faucet, Pos::Sink));
    neighbors.push(turn_on(state, Activatable::Stove, Pos::Stove));
    neighbors.push(turn_off(state, Activatable::Faucet, Pos::Sink));
    neighbors.push(turn_off(state, Activatable::Stove, Pos::Stove));
    neighbors.push(wait(state));

    let neighbors = neighbors
        .into_iter()
        .filter(|s| s.is_some())
        .map(|s| s.unwrap())
        .collect();

    neighbors
}

fn move_actor(state: &State, to: Pos) -> Option<Neighbor<State>> {
    if state.pos == to {
        return None;
    }

    let mut new_state = state.clone();
    new_state.pos = to;

    if new_state.holding == Some(Holdable::Pot) {
        new_state.pot_pos = to;
    }

    let action = match to {
        Pos::Sink => "Move to sink",
        Pos::Counter => "Move to counter",
        Pos::Stove => "Move to stove",
    };

    Some(Neighbor::new(new_state, 1.0, action.to_owned()))
}

fn pick_up(state: &State, holdable: Holdable) -> Option<Neighbor<State>> {
    let precondition = state.pos == state.pot_pos; // todo Check that holdable is pot
    if !precondition {
        return None;
    }

    let mut new_state = state.clone();
    new_state.holding = Some(holdable);
    Some(Neighbor::new(new_state, 1.0, "Pick up pot".to_owned()))
}

fn put_down(state: &State) -> Option<Neighbor<State>> {
    if state.holding.is_none() {
        return None;
    }

    let mut new_state = state.clone();
    new_state.holding = None;
    Some(Neighbor::new(new_state, 1.0, "Put down pot".to_owned()))
}

fn turn_on(state: &State, object: Activatable, required_pos: Pos) -> Option<Neighbor<State>> {
    if state.pos != required_pos {
        return None;
    }

    match object {
        Activatable::Stove => {
            if state.stove_on {
                None
            } else {
                let mut new_state = state.clone();
                new_state.stove_on = true;
                Some(Neighbor::new(new_state, 1.0, "Turn on stove".to_owned()))
            }
        }
        Activatable::Faucet => {
            if state.faucet_on {
                None
            } else {
                let mut new_state = state.clone();
                new_state.faucet_on = true;
                Some(Neighbor::new(new_state, 1.0, "Turn on faucet".to_owned()))
            }
        }
    }
}

fn turn_off(state: &State, object: Activatable, required_pos: Pos) -> Option<Neighbor<State>> {
    if state.pos != required_pos {
        return None;
    }

    match object {
        Activatable::Stove => {
            if !state.stove_on {
                None
            } else {
                let mut new_state = state.clone();
                new_state.stove_on = false;
                Some(Neighbor::new(new_state, 1.0, "Turn off stove".to_owned()))
            }
        }
        Activatable::Faucet => {
            if !state.faucet_on {
                None
            } else {
                let mut new_state = state.clone();
                new_state.faucet_on = false;
                Some(Neighbor::new(new_state, 1.0, "Turn off faucet".to_owned()))
            }
        }
    }
}

fn wait(state: &State) -> Option<Neighbor<State>> {
    let mut new_state = state.clone();

    if state.pot_pos == Pos::Sink && state.faucet_on {
        new_state.pot_filled = true;
    }

    Some(Neighbor::new(new_state, 1.0, "wait".to_owned()))
}

fn main() {
    let s0 = State {
        pos: Pos::Counter,
        pot_pos: Pos::Counter,
        pot_filled: false,
        faucet_on: false,
        stove_on: false,
        holding: None,
    };

    let result = a_star(&s0, &heuristic, &neighbors);

    match result {
        Ok(path) => {
            for (_, action) in path {
                println!("{}", action);
            }
        }
        Err(_) => println!("Unable to find solve!"),
    };
}
