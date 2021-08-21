use crate::{DoorId, PosId};
use crate::{Neighbor, State, World};
use serde::Deserialize;

#[derive(Debug, Clone)]
pub enum Action {
    Move { to: PosId },
    OpenDoor { door: DoorId },
    TraverseDoor { door: DoorId },
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
pub enum DoorState {
    Open,
    Closed,
    Locked,
    Broken,
}

pub fn move_actor(state: &State, world: &World, to: PosId) -> Option<Neighbor<State, Action>> {
    let precondition = (state.actor_pos != to)
        && (world.pos_move_groups[state.actor_pos] == world.pos_move_groups[to]);

    if !precondition {
        return None;
    }

    let mut new_state = state.clone();
    new_state.actor_pos = to;

    Some(Neighbor::new(new_state, 1.0, Action::Move { to }))
}

pub fn open_door(state: &State, world: &World, door: DoorId) -> Option<Neighbor<State, Action>> {
    let actor_pos = state.actor_pos;
    let door_side_a = world.door_side_a[door];
    let door_side_b = world.door_side_b[door];

    let precondition = (actor_pos == door_side_a || actor_pos == door_side_b)
        && state.door_states[door] == DoorState::Closed;

    if !precondition {
        return None;
    }

    let mut new_state = state.clone();
    new_state.door_states[door] = DoorState::Open;

    Some(Neighbor::new(new_state, 1.0, Action::OpenDoor { door }))
}

pub fn traverse_door(
    state: &State,
    world: &World,
    door: DoorId,
) -> Option<Neighbor<State, Action>> {
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
