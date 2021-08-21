mod a_star;
mod actions;

use a_star::{a_star, Neighbor};
use actions::{move_actor, open_door, traverse_door, Action, DoorState};

type PosId = usize;
type DoorId = usize;
type PosMoveGroupId = usize;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct State {
    pub actor_pos: PosId,
    pub door_states: Vec<DoorState>,
}

pub struct World {
    pub pos_move_groups: Vec<PosMoveGroupId>,
    pub door_side_a: Vec<DoorId>,
    pub door_side_b: Vec<DoorId>,
}

fn get_neighbors(state: &State, world: &World) -> Vec<Neighbor<State, Action>> {
    let mut actions = vec![];

    for i in 0..world.pos_move_groups.len() {
        actions.push(move_actor(state, world, i));
    }

    for i in 0..state.door_states.len() {
        actions.push(open_door(state, world, i));
    }

    for i in 0..state.door_states.len() {
        actions.push(traverse_door(state, world, i))
    }

    actions.into_iter().flatten().collect()
}

fn heuristic(state: &State) -> f32 {
    let mut distance = 0.0;

    if state.actor_pos != 9 {
        distance += 1.0;
    }

    distance
}

fn main() {
    let s0 = State {
        actor_pos: 0,
        door_states: vec![DoorState::Closed, DoorState::Closed],
    };

    let w = World {
        pos_move_groups: vec![1, 1, 1, 1, 2, 2, 2, 2, 3, 3],
        door_side_a: vec![3, 7],
        door_side_b: vec![4, 8],
    };

    let result = a_star(&s0, &heuristic, &|s: &State| get_neighbors(s, &w));

    match result {
        Ok(path) => {
            for (_, action) in path {
                println!("{:?}", action);
            }
        }
        Err(_) => println!("Unable to solve!"),
    };
}
