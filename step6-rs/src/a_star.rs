use std::collections::HashMap;

type Cost = f32;

#[derive(Debug)]
pub struct Neighbor<S, A> {
    state: S,
    cost: Cost,
    action: A,
}

impl<S, A> Neighbor<S, A> {
    pub fn new(state: S, cost: Cost, action: A) -> Neighbor<S, A> {
        Neighbor {
            state,
            cost,
            action,
        }
    }
}

pub fn a_star<S, A>(
    start: &S,
    heuristic: &dyn Fn(&S) -> f32,
    neighbors: &dyn Fn(&S) -> Vec<Neighbor<S, A>>,
) -> Result<Vec<(S, A)>, ()>
where
    S: Clone + PartialEq + Eq + std::hash::Hash + std::fmt::Debug,
    A: Clone,
{
    let mut open_set: Vec<S> = vec![start.clone()];

    let mut came_from = HashMap::new();

    // Cost of path from start to node
    let mut g_score = HashMap::new();
    g_score.insert(start.clone(), 0.0);

    // Estimated cost of path from start through node to goal
    // This is an estimate of the total path cost.
    let mut f_score = HashMap::new();
    f_score.insert(start.clone(), heuristic(start));

    while !open_set.is_empty() {
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

                if !open_set.contains(&neighbor.state) {
                    open_set.push(neighbor.state.clone())
                }
            }
        }
    }

    Err(())
}

fn reconstruct_path<S, A>(came_from: HashMap<S, (S, A)>, current: &S) -> Vec<(S, A)>
where
    S: Clone + Eq + std::hash::Hash,
    A: Clone,
{
    let mut total_path: Vec<(S, A)> = vec![];
    let mut current = current;

    while let Some((state, action)) = came_from.get(current) {
        total_path.insert(0, (state.clone(), action.to_owned()));
        current = state;
    }

    total_path
}
