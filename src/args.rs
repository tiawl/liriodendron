extern crate structopt;
use structopt::StructOpt;

const DEFAULT_MAX_ACTIONS: &str = "8";
const DEFAULT_MAX_FILLED_CELLS: &str = "7";
const DEFAULT_MAX_WAITING_TASKS: &str = "2";
const DEFAULT_THREADS_NUMBER: &str = "4";

/// StructOpt struct
#[derive(StructOpt)]
#[structopt(
  name = "Liriodendron",
  version = "0.2.0",
  author = "Pablo TOMAS",
  about = "Procedural Pixellized 2D Textures Generator"
)]
pub struct Args {
  /// Open a new workspace in the terminal and use it as user interface
  #[structopt(short = "u", long = "userinterface", help = "Uses terminal to \
    edit grids and generate textures")]
  user_interface: bool,

  /// Set the maximum number of saved actions in the <i>actions</i> member
  /// of <i>log</i>
  #[structopt(short = "a", long = "max-actions",
    default_value = DEFAULT_MAX_ACTIONS, help = "Set the maximum number of \
    saved actions in the log")]
  max_actions: u32,

  /// Set the maximum number of saved filled cells in the <i>state</i> member
  /// of <i>log</i> grids
  #[structopt(short = "c", long = "max-filled-cells",
    default_value = DEFAULT_MAX_FILLED_CELLS, help = "Set the maximum number \
    of brushed cells on the grids")]
  max_filled_cells: u32,

  /// Set the maximum number of waiting tasks in the <i>tasksmanager</i>'s
  /// tasks queue
  #[structopt(short = "w", long = "max-waiting-tasks",
    default_value = DEFAULT_MAX_WAITING_TASKS, help = "Set the maximum \
    number of possible waiting tasks")]
  max_waiting_tasks: u32,

  /// Set the number of threads in the <i>tasksmanager</i>'s threadpool
  #[structopt(short = "t", long = "threads-number",
    default_value = DEFAULT_THREADS_NUMBER, help = "Set the maximum number \
    of running tasks threads")]
  threads_number: usize,
}

impl Args {
  pub fn is_ui_used(&self) -> bool {
    self.user_interface
  }

  pub fn get_threads_number(&self) -> Result<usize, std::io::Error> {
    if (self.threads_number > 8) || (self.threads_number < 1) {
      Err(std::io::Error::new(std::io::ErrorKind::InvalidInput,
        "threads number argument must be in this interval [1; 8]."))
    } else {
      Ok(self.threads_number)
    }
  }

  pub fn get_max_waiting_tasks(&self) -> Result<usize, std::io::Error> {
    if (self.max_waiting_tasks > 7) || (self.max_waiting_tasks < 2) {
      Err(std::io::Error::new(std::io::ErrorKind::InvalidInput,
        "maximum waiting tasks argument must be in this interval [2; 7]."))
    } else {
      Ok(2usize.pow(self.max_waiting_tasks))
    }
  }

  pub fn get_max_actions(&self) -> Result<usize, std::io::Error> {
    if (self.max_actions > 12) || (self.max_actions < 5) {
      Err(std::io::Error::new(std::io::ErrorKind::InvalidInput,
        "maximum actions argument must be in this interval [5; 12]."))
    } else {
      Ok(2usize.pow(self.max_actions))
    }
  }

  pub fn get_max_filled_cells(&self) -> Result<usize, std::io::Error> {
    if (self.max_filled_cells > 16) || (self.max_filled_cells < 7) {
      Err(std::io::Error::new(std::io::ErrorKind::InvalidInput,
        "maximum filled cells argument must be in this interval [7; 16]."))
    } else {
      Ok(2usize.pow(self.max_filled_cells))
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_uses_ui() {
    let args = Args {
      user_interface: true,
      max_actions: DEFAULT_MAX_ACTIONS.parse::<u32>().unwrap(),
      max_filled_cells: DEFAULT_MAX_FILLED_CELLS.parse::<u32>().unwrap(),
      max_waiting_tasks: DEFAULT_MAX_WAITING_TASKS.parse::<u32>().unwrap(),
      threads_number: DEFAULT_THREADS_NUMBER.parse::<usize>().unwrap(),
    };
    assert!(args.is_ui_used())
  }

  #[test]
  fn it_does_not_use_ui() {
    let args = Args {
      user_interface: false,
      max_actions: DEFAULT_MAX_ACTIONS.parse::<u32>().unwrap(),
      max_filled_cells: DEFAULT_MAX_FILLED_CELLS.parse::<u32>().unwrap(),
      max_waiting_tasks: DEFAULT_MAX_WAITING_TASKS.parse::<u32>().unwrap(),
      threads_number: DEFAULT_THREADS_NUMBER.parse::<usize>().unwrap(),
    };
    assert!(!args.is_ui_used())
  }
}
