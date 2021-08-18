use std::collections::HashMap;

#[derive(Debug)]
pub struct Neighbor<S> {
    state: S,
    cost: f32,
    action: String,
}

impl<S> Neighbor<S> {
    pub fn new(state: S, cost: f32, action: String) -> Neighbor<S> {
        Neighbor {
            state,
            cost,
            action,
        }
    }
}

pub fn a_star<S>(
    start: &S,
    heuristic: &dyn Fn(&S) -> f32,
    neighbors: &dyn Fn(&S) -> Vec<Neighbor<S>>,
) -> Result<Vec<(S, String)>, ()>
where
    S: Clone + PartialEq + Eq + std::hash::Hash + std::fmt::Debug,
{
    let mut open_set: Vec<S> = vec![start.clone()];

    let mut came_from = HashMap::new();

    // Cost of path from start to node
    let mut g_score = HashMap::new();
    g_score.insert(start.clone(), 0.0);

    // Estimated cost of path from start through node to goal
    // This is an estimate of the total path cost.
    let mut f_score = HashMap::new();
    f_score.insert(start.clone(), heuristic(&start));

    while open_set.len() > 0 {
        open_set.sort_by(|a, b| {
            f_score
                .get(a)
                .unwrap_or(&f32::INFINITY)
                .partial_cmp(f_score.get(b).unwrap_or(&f32::INFINITY))
                .unwrap()
        });

        let current = &open_set.remove(0);

        if heuristic(current) == 0.0 {
            return Ok(reconstruct_path(came_from, current));
        }

        for neighbor in neighbors(current) {
            let tentative_g_score = g_score.get(current).unwrap_or(&f32::INFINITY) + neighbor.cost;
            if tentative_g_score < *g_score.get(&neighbor.state).unwrap_or(&f32::INFINITY) {
                // This path to the neighbor is the best one seen so far
                came_from.insert(
                    neighbor.state.clone(),
                    (current.clone(), neighbor.action.to_owned()),
                );
                g_score.insert(neighbor.state.clone(), tentative_g_score);
                f_score.insert(
                    neighbor.state.clone(),
                    tentative_g_score + heuristic(&neighbor.state),
                );

                if !open_set.contains(&&neighbor.state) {
                    open_set.push(neighbor.state.clone())
                }
            }
        }
    }

    return Err(());
}

fn reconstruct_path<S>(came_from: HashMap<S, (S, String)>, current: &S) -> Vec<(S, String)>
where
    S: Clone + Eq + std::hash::Hash,
{
    let mut total_path: Vec<(S, String)> = vec![(current.clone(), "goal".to_owned())];
    let mut current = current;

    while let Some((state, action)) = came_from.get(current) {
        total_path.insert(0, (state.clone(), action.to_owned()));
        current = state;
    }

    total_path
}
