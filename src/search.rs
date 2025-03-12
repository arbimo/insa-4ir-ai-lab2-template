use rand::Rng; // import trait to make the `random_range` method available (Rng = Random number generator)

use crate::board::*;

pub fn select_action(board: PlayableBoard) -> Option<Action> {
    select_action_randomly(board)
    // select_action_greedily(board)
}

pub fn select_action_randomly(board: PlayableBoard) -> Option<Action> {
    // iterate through all actions and keep the applicable ones
    let mut applicable_actions: Vec<Action> = Vec::new();
    for action in ALL_ACTIONS {
        if let Some(_succ) = board.apply(action) {
            // action is applicable
            applicable_actions.push(action);
        } else {
            // action is not aplicable, ignore
        }
    }

    // if there is no available actions, return `None` immediately
    let num_actions = applicable_actions.len();
    if num_actions == 0 {
        // no available action
        return None;
    }

    // otherwise, randomly pick an action among the applicable ones
    let randomly_selected_action_index = rand::rng().random_range(0..num_actions);
    let randomly_selected_action = applicable_actions[randomly_selected_action_index];
    Some(randomly_selected_action)
}

pub fn select_action_greedily(board: PlayableBoard) -> Option<Action> {
    // DO NOT COPY PAST from select_action_randomly
    // You can use for inspiration on how to use the API, but the selection process is fairly different
    todo!()
}

pub fn select_action_expectimax(board: PlayableBoard, max_actions: usize) -> Option<Action> {
    let mut stats = Stats::default();
    todo!()
}

fn evaluate_randable(board: RandableBoard, remaining_actions: usize, stats: &mut Stats) -> f32 {
    todo!()
}

fn evaluate_playable(board: PlayableBoard, remaining_actions: usize, stats: &mut Stats) -> f32 {
    todo!()
}

/// A small structure to accumulated statistics accros deeply nested calls
#[derive(Default)]
struct Stats {
    /// number of time the evaluation method is called on
    pub num_evals: usize,
}

impl std::fmt::Display for Stats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Num evals: {}", self.num_evals)?;
        Ok(())
    }
}
