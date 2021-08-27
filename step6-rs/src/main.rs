mod a_star;
mod actions;

use a_star::{a_star, Neighbor};
use actions::{close_door, move_actor, open_door, traverse_door, Action, DoorState};
use serde::Deserialize;
use std::fs::read_to_string;

type PosId = usize;
type DoorId = usize;
type PosMoveGroupId = usize;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
pub struct State {
    pub actor_pos: PosId,
    pub door_states: Vec<DoorState>,
}

#[derive(Deserialize)]
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
        actions.push(close_door(state, world, i));
    }

    for i in 0..state.door_states.len() {
        actions.push(traverse_door(state, world, i))
    }

    actions.into_iter().flatten().collect()
}

fn heuristic(state: &State, objectives: &[Objective]) -> f32 {
    let mut distance = 0.0;

    for obj in objectives {
        match obj {
            Objective::ActorPos(pos_id) => {
                if state.actor_pos != *pos_id {
                    distance += 1.0;
                }
            }
            Objective::DoorState(door_id, door_state) => {
                if state.door_states[*door_id] != *door_state {
                    distance += 1.0;
                }
            }
        }
    }

    distance
}

#[derive(Deserialize)]
enum Objective {
    ActorPos(PosId),
    DoorState(DoorId, DoorState),
}

#[derive(Deserialize)]
struct Scenario {
    world: World,
    state: State,
    objectives: Vec<Objective>,
}

fn main() {
    let data = read_to_string("scenario.json").unwrap();
    let scenario: Scenario = serde_json::from_str(&data).unwrap();
    let s0 = scenario.state;
    let w = scenario.world;
    let o = scenario.objectives;

    let result = a_star(&s0, &|s| heuristic(s, &o), &|s: &State| {
        get_neighbors(s, &w)
    });

    match result {
        Ok(path) => {
            for (_, action) in path {
                println!("{:?}", action);
            }
        }
        Err(_) => println!("Unable to solve!"),
    };
}
