extern crate futures;
use futures::future::{Future, lazy};

extern crate std;
use std::collections::{HashMap, VecDeque};
use std::marker::Send;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

extern crate tokio_threadpool;
use tokio_threadpool::Builder;

use crate::log;
use crate::task::{Task, generation};

const TASKS_QUEUE_FULL: bool = true;

/// Multi-threaded struct to support expensive users queries and avoid
/// blocking tasks
pub struct TasksManager {

  /// Running threads counter
  nb_running_threads: Arc<Mutex<usize>>,

  /// Queue's length
  waiting_tasks: Arc<Mutex<usize>>,

  /// When <i>true</i>, threads send their progression
  is_sending: Arc<Mutex<bool>>,

  threadpool: tokio_threadpool::ThreadPool,
  sender: Sender<Option<(thread::ThreadId, (u16, u16))>>,
  receiver: Receiver<Option<(thread::ThreadId, (u16, u16))>>,

  /// Waiting tasks are stocked in this queue
  tasks: VecDeque::<Box<dyn Task + Send>>,

  todo_tasks: usize,
  done_tasks: usize,

  /// Stocks threads' progression
  threads_monitor: HashMap::<thread::ThreadId, (u16, u16)>,

  /// Customizable member
  threads_number: usize,

  /// Customizable member
  max_tasks: usize,
}

impl TasksManager {

  pub fn new(threads_number: usize, max_tasks: usize) -> TasksManager {
    let (sender, receiver): (Sender<Option<(thread::ThreadId, (u16, u16))>>,
      Receiver<Option<(thread::ThreadId, (u16, u16))>>) = channel();
    TasksManager {
      nb_running_threads: Arc::new(Mutex::new(0)),
      waiting_tasks: Arc::new(Mutex::new(0)),
      is_sending: Arc::new(Mutex::new(false)),
      threadpool: Builder::new().pool_size(threads_number).build(),
      sender: sender,
      receiver: receiver,

      tasks:
        VecDeque::<Box<dyn Task + Send>>::with_capacity(max_tasks),
      todo_tasks: 0,
      done_tasks: 0,
      threads_monitor: HashMap::<thread::ThreadId, (u16, u16)>::with_capacity(
        threads_number + max_tasks),

      threads_number: threads_number,
      max_tasks: max_tasks,
    }
  }

  pub fn new_generation_task(&mut self, log: &log::Log) ->
    (std::io::Result<()>, bool) {
      let res = generation::Generation::new(log);
      match res {
        Ok(generation) => {
          if self.tasks.len() < self.max_tasks {
            let mut lock = self.waiting_tasks.lock().unwrap();
            self.tasks.push_back(Box::new(generation));
            *lock = self.tasks.len();
            std::mem::drop(lock);
            return (Ok(()), TASKS_QUEUE_FULL)
          } else {
            return (Ok(()), !TASKS_QUEUE_FULL)
          }
        },
        Err(error) => return (Err(error), TASKS_QUEUE_FULL)
      }
  }

  fn get_nb_running_threads(&self) -> usize {
    let lock = self.nb_running_threads.lock().unwrap();
    let nb_running_threads = *lock;
    std::mem::drop(lock);
    nb_running_threads
  }

  pub fn get_todo_and_max_tasks(&self) -> (usize, usize) {
    (self.get_nb_running_threads() + self.tasks.len(),
      self.max_tasks + self.threads_number)
  }

  pub fn get_done_and_todo_tasks(&self) -> (usize, usize) {
    (self.done_tasks, self.todo_tasks)
  }

  /// Checks if a task is in the queue. If a thread is available, it runs
  /// the task and pop it from the queue.
  pub fn check_queue(&mut self) {
    if self.get_nb_running_threads() < self.threads_number {
      if let Some(task) = self.tasks.pop_front() {

        // These 2 mutexes are locked in the same time to not allow
        // waiting_tasks = 0 and nb_running_threads = 0 while there are
        // a last task in the queue
        let mut lock = self.waiting_tasks.lock().unwrap();
        let mut running_threads = self.nb_running_threads.lock().unwrap();
        *running_threads += 1;
        *lock = self.tasks.len();
        std::mem::drop(running_threads);
        std::mem::drop(lock);

        let thread_sender = Sender::clone(&self.sender);
        let mutex_threads = Arc::clone(&self.nb_running_threads);
        let mutex_issending = Arc::clone(&self.is_sending);

        self.threadpool.spawn(lazy(move || {

          let mut current_subtask = 1;
          loop {
            if current_subtask > task.get_nb_subtasks() {
              break
            }
            task.run(current_subtask);
            let is_sending = mutex_issending.lock().unwrap();
            if *is_sending {
              thread_sender.send(Some((thread::current().id(),
                (current_subtask, task.get_nb_subtasks())))).unwrap();
            }
            std::mem::drop(is_sending);
            current_subtask += 1;
          }

          let mut running_threads = mutex_threads.lock().unwrap();
          *running_threads -= 1;
          std::mem::drop(running_threads);

          Ok(())
        }));
      }
    }
  }

  pub fn start_sender_channel(&mut self) {
    let mut is_sending = self.is_sending.lock().unwrap();
    *is_sending = true;
    std::mem::drop(is_sending);
  }

  pub fn compute_todotasks(&mut self) {
    self.todo_tasks = self.get_nb_running_threads() + self.tasks.len();
  }

  /// Unlocks a blocking std::sync::mpsc::Receiver.recv() call
  pub fn spawn_unlocking_thread(&self) {
    let thread_sender = Sender::clone(&self.sender);
    let mutex_threads = Arc::clone(&self.nb_running_threads);
    let mutex_tasks = Arc::clone(&self.waiting_tasks);
    thread::spawn(move || {
      let mut break_loop;
      loop {
        break_loop = true;

        // These 2 mutexes are locked in the same time to not allow
        // waiting_tasks = 0 and running_threads = 0 while there are
        // a last task
        let waiting_tasks = mutex_tasks.lock().unwrap();
        let running_threads = mutex_threads.lock().unwrap();
        break_loop = break_loop && (*running_threads == 0);
        break_loop = break_loop && (*waiting_tasks == 0);
        std::mem::drop(running_threads);
        std::mem::drop(waiting_tasks);

        if break_loop {
          thread_sender.send(None).unwrap();
          break;
        }
      }
    });
  }

  pub fn shutdown(self) {
    self.threadpool.shutdown().wait().unwrap();
  }

  pub fn get_threads_monitor(&self) -> &HashMap<thread::ThreadId, (u16, u16)> {
    &self.threads_monitor
  }

  /// While the receiver is receiving data from the threadpool, a thread is
  /// running
  pub fn is_running(&mut self) -> bool {
    let is_receiving;
    if let Ok(Some((thread_id, (gen_done, gen_todo)))) =
      self.receiver.recv() {
        is_receiving = true;
        self.threads_monitor.insert(thread_id, (gen_done, gen_todo));
        if gen_done == gen_todo {
          self.threads_monitor.remove(&thread_id);
          self.done_tasks += 1;
        }
    } else {
      is_receiving = false;
    }
    is_receiving
  }
}

#[cfg(test)]
mod tests {

  use super::*;
  use std::time;
  use std::fs;

  fn remove_generation_dirs() {
    let paths = fs::read_dir("./").unwrap();
    let generation_dir_name = "./liriodendron_generation";

    let generation_directories: Vec<fs::DirEntry> =
      paths
      .filter(|path|
        match path {
          Ok(_) => return true,
          Err(_) => return false,
        }
      )
      .map(|path| path.unwrap())
      .filter(|path| match path.path().to_str() {
        Some(str_path) => {
          if str_path.len() >= 25 {
            return generation_dir_name.eq(&str_path[0..25])
          } else {
            return false
          }
        },
        _ => return false,
      }).collect();
    for dir in generation_directories.iter() {
      fs::remove_dir_all(dir.path()).unwrap();
    }
  }

  fn is_single_threaded_test(args: std::env::Args) -> bool {
    let mut is_single_threaded = false;
    let single_threaded_flag = "--test-threads=1";
    for argument in args {
      is_single_threaded =
        is_single_threaded || argument.eq(&single_threaded_flag);
    }
    is_single_threaded
  }

  #[test]
  fn it_adds_a_new_generation_task() {
    if is_single_threaded_test(std::env::args()) {
      let log = log::Log::new(0, 0);
      let mut tasksmanager = TasksManager::new(1, 1);
      let init_len = tasksmanager.tasks.len();
      let (_, task_added) = tasksmanager.new_generation_task(&log);
      assert!((init_len == 0) && (tasksmanager.tasks.len() == 1) &&
        task_added);
      remove_generation_dirs();
    } else {
      assert!(false, "tests in taskmanager module must be run with this \
        cargo test option: \"--test-threads=1\"");
    }
  }

  #[test]
  fn it_does_not_add_a_new_generation_task_when_task_queue_is_full() {
    if is_single_threaded_test(std::env::args()) {
      let log = log::Log::new(0, 0);
      let mut tasksmanager = TasksManager::new(1, 1);
      let (_, task_added1) = tasksmanager.new_generation_task(&log);
      let init_len = tasksmanager.tasks.len();
      let (_, task_added2) = tasksmanager.new_generation_task(&log);
      assert!((init_len == 1) && (tasksmanager.tasks.len() == 1) &&
        task_added1 && !task_added2);
      remove_generation_dirs();
    } else {
      assert!(false, "tests in taskmanager module must be run with this \
        cargo test option: \"--test-threads=1\"");
    }
  }

  #[test]
  fn it_runs_a_generation_task() {
    if is_single_threaded_test(std::env::args()) {
      let log = log::Log::new(0, 0);
      let mut tasksmanager = TasksManager::new(1, 1);
      let (_, task_added) = tasksmanager.new_generation_task(&log);
      let init_len = tasksmanager.tasks.len();
      tasksmanager.check_queue();
      assert!((init_len == 1) && (tasksmanager.tasks.len() == 0) &&
        task_added);
      tasksmanager.shutdown();
      remove_generation_dirs();
    } else {
      assert!(false, "tests in taskmanager module must be run with this \
        cargo test option: \"--test-threads=1\"");
    }
  }

  #[test]
  fn it_can_not_run_a_generation_task_because_threads_are_busy() {
    if is_single_threaded_test(std::env::args()) {
      let log = log::Log::new(0, 0);
      let mut tasksmanager = TasksManager::new(1, 2);
      let (_, task_added1) = tasksmanager.new_generation_task(&log);
      let (_, task_added2) = tasksmanager.new_generation_task(&log);
      let init_len = tasksmanager.tasks.len();

      let (sender, receiver) = channel();

      let mutex_issending = Arc::clone(&tasksmanager.is_sending);
      let simulate_heavy_task = thread::spawn(move || {
        let is_sending = mutex_issending.lock().unwrap();
        receiver.recv().unwrap();
        std::mem::drop(is_sending);
      });

      tasksmanager.check_queue();
      let second_len = tasksmanager.tasks.len();

      let waiting_time = time::Duration::from_millis(100);
      thread::sleep(waiting_time);

      tasksmanager.check_queue();
      let third_len = tasksmanager.tasks.len();

      sender.send(()).unwrap();
      simulate_heavy_task.join().unwrap();

      tasksmanager.check_queue();

      assert!((init_len == 2) && (second_len == 1) && (third_len == 1) &&
        (tasksmanager.tasks.len() == 0) && task_added1 && task_added2);
      tasksmanager.shutdown();
      remove_generation_dirs();
    } else {
      assert!(false, "tests in taskmanager module must be run with this \
        cargo test option: \"--test-threads=1\"");
    }
  }

  #[test]
  fn it_spawns_a_thread_to_unlock_a_blocking_recv_call() {
    if is_single_threaded_test(std::env::args()) {
      let tasksmanager = TasksManager::new(1, 1);
      tasksmanager.spawn_unlocking_thread();
      let unlock_blocking_recv = match tasksmanager.receiver.recv().unwrap() {
        None => true,
        _ => false,
      };
      assert!(unlock_blocking_recv);
    } else {
      assert!(false, "tests in taskmanager module must be run with this \
        cargo test option: \"--test-threads=1\"");
    }
  }

  #[test]
  fn it_spawns_threads_that_does_not_use_sender_channel() {
    if is_single_threaded_test(std::env::args()) {
      let log = log::Log::new(0, 0);
      let mut tasksmanager = TasksManager::new(1, 2);
      let (_, task_added) = tasksmanager.new_generation_task(&log);
      tasksmanager.check_queue();
      tasksmanager.spawn_unlocking_thread();
      let threads_are_using_sender_channel =
        match tasksmanager.receiver.recv().unwrap() {
          None => false,
          _ => true,
        };
      assert!(!threads_are_using_sender_channel && task_added);
      tasksmanager.shutdown();
      remove_generation_dirs();
    } else {
      assert!(false, "tests in taskmanager module must be run with this \
        cargo test option: \"--test-threads=1\"");
    }
  }

  #[test]
  fn it_spawns_threads_using_sender_channel() {
    if is_single_threaded_test(std::env::args()) {
      let log = log::Log::new(0, 0);
      let mut tasksmanager = TasksManager::new(1, 2);
      let (_, task_added) = tasksmanager.new_generation_task(&log);
      tasksmanager.start_sender_channel();
      tasksmanager.check_queue();
      let threads_are_using_sender_channel =
        match tasksmanager.receiver.recv().unwrap() {
          None => false,
          _ => true,
        };
      assert!(threads_are_using_sender_channel && task_added);
      tasksmanager.shutdown();
      remove_generation_dirs();
    } else {
      assert!(false, "tests in taskmanager module must be run with this \
        cargo test option: \"--test-threads=1\"");
    }
  }

  #[test]
  fn it_receives_all_data_sended_by_threads_with_a_singlethreaded_threadpool() {
    if is_single_threaded_test(std::env::args()) {
      let log = log::Log::new(0, 0);
      let mut tasksmanager = TasksManager::new(1, 5);
      let (_, task_added1) = tasksmanager.new_generation_task(&log);
      let (_, task_added2) = tasksmanager.new_generation_task(&log);
      let (_, task_added3) = tasksmanager.new_generation_task(&log);
      let (_, task_added4) = tasksmanager.new_generation_task(&log);
      let (_, task_added5) = tasksmanager.new_generation_task(&log);
      tasksmanager.start_sender_channel();
      while tasksmanager.tasks.len() > 0 {
        tasksmanager.check_queue();
      }
      let mut received_counter = 0;
      while tasksmanager.is_running() {
        received_counter += 1;
        if received_counter == 5 {
          tasksmanager.spawn_unlocking_thread();
        }
      }
      assert!((received_counter == 5) && task_added1 && task_added2 &&
        task_added3 && task_added4 && task_added5);
      tasksmanager.shutdown();
      remove_generation_dirs();
    } else {
      assert!(false, "tests in taskmanager module must be run with this \
        cargo test option: \"--test-threads=1\"");
    }
  }

  #[test]
  fn it_receives_all_data_sended_by_threads_with_a_multiplethreaded_threadpool() {
    if is_single_threaded_test(std::env::args()) {
      let log = log::Log::new(0, 0);
      let mut tasksmanager = TasksManager::new(5, 5);
      let (_, task_added1) = tasksmanager.new_generation_task(&log);
      let (_, task_added2) = tasksmanager.new_generation_task(&log);
      let (_, task_added3) = tasksmanager.new_generation_task(&log);
      let (_, task_added4) = tasksmanager.new_generation_task(&log);
      let (_, task_added5) = tasksmanager.new_generation_task(&log);
      tasksmanager.start_sender_channel();
      while tasksmanager.tasks.len() > 0 {
        tasksmanager.check_queue();
      }
      let mut received_counter = 0;
      while tasksmanager.is_running() {
        received_counter += 1;
        if received_counter == 5 {
          tasksmanager.spawn_unlocking_thread();
        }
      }
      assert!((received_counter == 5) && task_added1 && task_added2 &&
        task_added3 && task_added4 && task_added5);
      tasksmanager.shutdown();
      remove_generation_dirs();
    } else {
      assert!(false, "tests in taskmanager module must be run with this \
        cargo test option: \"--test-threads=1\"");
    }
  }
}
