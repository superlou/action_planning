#[macro_use]
extern crate rocket;

mod a_star;
mod actions;

use a_star::{a_star, Neighbor};
use actions::{close_door, move_actor, open_door, traverse_door, Action, DoorState};
use rocket::serde::json::Json;
use rocket::{Build, Rocket};
use serde::{Deserialize, Serialize};
use std::fs::read_to_string;

type PosId = usize;
type DoorId = usize;
type PosMoveGroupId = usize;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
pub struct State {
    pub actor_pos: PosId,
    pub door_states: Vec<DoorState>,
}

#[derive(Deserialize, Debug)]
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

#[derive(Deserialize, Debug)]
enum Objective {
    ActorPos(PosId),
    DoorState(DoorId, DoorState),
}

#[derive(Deserialize, Debug)]
struct Scenario {
    world: World,
    state: State,
    objectives: Vec<Objective>,
}

#[derive(Debug, Serialize)]
struct PlannerResult {
    actions: Vec<Action>,
    success: bool,
}

fn run_scenario_from_file(scenario_filename: &str) {
    let data = read_to_string(scenario_filename).unwrap();
    let scenario: Scenario = serde_json::from_str(&data).unwrap();
    run_scenario(scenario);
}

fn run_scenario(scenario: Scenario) -> PlannerResult {
    let s0 = scenario.state;
    let w = scenario.world;
    let o = scenario.objectives;

    let result = a_star(&s0, &|s| heuristic(s, &o), &|s: &State| {
        get_neighbors(s, &w)
    });

    let planner_result = match result {
        Ok(result) => {
            let actions = result.into_iter().map(|(_, action)| action).collect();
            PlannerResult {
                actions,
                success: true,
            }
        }
        Err(_) => PlannerResult {
            actions: vec![],
            success: false,
        },
    };

    planner_result
}

#[post("/", data = "<scenario>")]
fn index(scenario: Json<Scenario>) -> Json<PlannerResult> {
    let scenario = scenario.into_inner();
    let planner_result = run_scenario(scenario);
    Json(planner_result)
}

fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![index])
}

#[rocket::main]
async fn main() {
    let input_file = std::env::args().nth(1);

    match input_file {
        Some(x) => run_scenario_from_file(&x),
        None => {
            let _ = rocket().launch().await;
        }
    }
}
