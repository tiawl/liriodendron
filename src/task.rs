//! Run user queries and manage them in threads if queries are expensive

/// Module for generation tasks
pub mod generation;

/// Module for threads and user's tasks management
pub mod tasksmanager;

trait Task {
  fn run(&self, current_subtask: u16);
  fn get_nb_subtasks(&self) -> u16;
}
