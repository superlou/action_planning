mod a_star;

use a_star::{a_star, Neighbor};

#[derive(Debug, Clone)]
pub enum Action {
    Move { from: usize, to: usize },
}

type PosId = usize;

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
    pub actor_pos: PosId,
}

pub struct World {
    pub pos_move_groups: Vec<usize>,
}

fn heuristic(state: &State) -> f32 {
    let mut distance = 0.0;

    if state.actor_pos != 3 {
        distance += 1.0;
    }

    distance
}

fn neighbors(state: &State, world: &World) -> Vec<Neighbor<State, Action>> {
    let mut neighbors = vec![];

    for i in 0..8 {
        neighbors.push(move_actor(state, world, i));
    }

    let neighbors = neighbors
        .into_iter()
        .filter(|s| s.is_some())
        .map(|s| s.unwrap())
        .collect();

    neighbors
}

fn move_actor(state: &State, world: &World, to: PosId) -> Option<Neighbor<State, Action>> {
    if state.actor_pos == to {
        return None;
    }

    if world.pos_move_groups[state.actor_pos] != world.pos_move_groups[to] {
        return None;
    }

    let mut new_state = state.clone();
    new_state.actor_pos = to;

    Some(Neighbor::new(
        new_state,
        1.0,
        Action::Move {
            from: state.actor_pos,
            to,
        },
    ))
}

fn main() {
    let s0 = State { actor_pos: 0 };
    let w = World {
        pos_move_groups: vec![1, 1, 1, 1, 2, 2, 2, 2],
    };

    let result = a_star(&s0, &heuristic, &|s: &State| neighbors(s, &w));

    match result {
        Ok(path) => {
            for (_, action) in path {
                println!("{:?}", action);
            }
        }
        Err(_) => println!("Unable to solve!"),
    };
}
