mod a_star;

use a_star::{a_star, Neighbor};

type PosId = usize;

#[derive(Debug, Clone)]
pub enum Action {
    Move { from: usize, to: usize },
    OpenDoor { door: usize },
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
    pub pos_move_groups: Vec<usize>,
    pub door_side_a: Vec<usize>,
    pub door_side_b: Vec<usize>,
}

fn heuristic(state: &State) -> f32 {
    let mut distance = 0.0;

    if state.actor_pos != 2 {
        distance += 1.0;
    }

    if state.door_states[0] != DoorState::Open {
        distance += 1.0;
    }

    distance
}

fn neighbors(state: &State, world: &World) -> Vec<Neighbor<State, Action>> {
    let mut neighbors = vec![];

    for i in 0..8 {
        neighbors.push(move_actor(state, world, i));
    }

    for i in 0..1 {
        neighbors.push(open_door(state, world, i));
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

fn open_door(state: &State, world: &World, door: usize) -> Option<Neighbor<State, Action>> {
    let prec = ((state.actor_pos == world.door_side_a[door])
        || (state.actor_pos == world.door_side_a[door]))
        && state.door_states[door] == DoorState::Closed;
    if !prec {
        return None;
    }

    let mut new_state = state.clone();
    new_state.door_states[door] = DoorState::Open;

    Some(Neighbor::new(new_state, 1.0, Action::OpenDoor { door }))
}

// fn traverse_door(state: &State, world: &World, to: PosId) -> Option<Neighbor<State, Action>> {}
//
// trait Action2 {
//     fn precondition(state: &State, world: &World) -> bool;
//     fn perform(state: &State, world: &World) -> Neighbor<State, Action>;
//     fn realize(&self) -> Option<Neighbor<State, Action>> {
//         None
//     }
// }
//
// struct MoveActor {
//     to: usize,
// }
//
// impl MoveActor {
//     pub fn new(to: usize) -> Self {
//         Self { to }
//     }
// }
//
// impl Action2 for MoveActor {
//     fn precondition(state: &State, world: &World) -> bool {
//         true
//     }
// }
//
// fn try_using() {
//     let a0 = MoveActor::new(0);
//     let a1 = MoveActor::new(1);
//
//     let actions = vec![a0, a1];
// }

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
