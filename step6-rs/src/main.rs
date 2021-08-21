mod a_star;

use a_star::{a_star, Neighbor};

type PosId = usize;
type DoorId = usize;
type PosMoveGroupId = usize;

#[derive(Debug, Clone)]
pub enum Action {
    Move { to: PosId },
    OpenDoor { door: DoorId },
    TraverseDoor { door: DoorId },
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum DoorState {
    Open,
    Closed,
    Locked,
    Broken,
}

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

fn heuristic(state: &State) -> f32 {
    let mut distance = 0.0;

    if state.actor_pos != 5 {
        distance += 1.0;
    }

    if state.door_states[0] != DoorState::Open {
        distance += 1.0;
    }

    distance
}

fn neighbors(state: &State, world: &World) -> Vec<Neighbor<State, Action>> {
    let mut actions = vec![];

    for i in 0..8 {
        actions.push(move_actor(state, world, i));
    }

    for i in 0..1 {
        actions.push(open_door(state, world, i));
    }

    for i in 0..1 {
        actions.push(traverse_door(state, world, i))
    }

    actions.into_iter().flatten().collect()
}

fn move_actor(state: &State, world: &World, to: PosId) -> Option<Neighbor<State, Action>> {
    let precondition = (state.actor_pos != to)
        && (world.pos_move_groups[state.actor_pos] == world.pos_move_groups[to]);

    if !precondition {
        return None;
    }

    let mut new_state = state.clone();
    new_state.actor_pos = to;

    Some(Neighbor::new(new_state, 1.0, Action::Move { to }))
}

fn open_door(state: &State, world: &World, door: DoorId) -> Option<Neighbor<State, Action>> {
    let precondition = ((state.actor_pos == world.door_side_a[door])
        || (state.actor_pos == world.door_side_a[door]))
        && state.door_states[door] == DoorState::Closed;

    if !precondition {
        return None;
    }

    let mut new_state = state.clone();
    new_state.door_states[door] = DoorState::Open;

    Some(Neighbor::new(new_state, 1.0, Action::OpenDoor { door }))
}

fn traverse_door(state: &State, world: &World, door: DoorId) -> Option<Neighbor<State, Action>> {
    let actor_pos = state.actor_pos;
    let door_side_a = world.door_side_a[door];
    let door_side_b = world.door_side_b[door];

    let precondition = (actor_pos == door_side_a || actor_pos == door_side_b)
        && state.door_states[door] == DoorState::Open;

    if !precondition {
        return None;
    }

    let new_pos = if actor_pos == door_side_a {
        door_side_b
    } else {
        door_side_a
    };

    let mut new_state = state.clone();
    new_state.actor_pos = new_pos;
    Some(Neighbor::new(new_state, 1.0, Action::TraverseDoor { door }))
}

fn main() {
    let s0 = State {
        actor_pos: 0,
        door_states: vec![DoorState::Closed],
    };

    let w = World {
        pos_move_groups: vec![1, 1, 1, 1, 2, 2, 2, 2],
        door_side_a: vec![3],
        door_side_b: vec![4],
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
