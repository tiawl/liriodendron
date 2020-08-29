//! # liriodendron
//!
//! `liriodendron` is the module which have the responsability to run the
//! main function.

extern crate better_panic;

extern crate std;
use std::panic;

extern crate structopt;
use structopt::StructOpt;

extern crate tui;
use tui::Terminal;
use tui::backend::CrosstermBackend;

mod args;
mod log;
mod userinterface;
mod utils;
mod task;

use userinterface::{errors, events, views};

/// Modifies panic behavior
fn custom_panic() {
  panic::set_hook(Box::new(|panic_info| {
    userinterface::finish();
    better_panic::Settings::auto().create_panic_handler()(panic_info);
  }));
}

fn main() {

  better_panic::install();

  let args = args::Args::from_args();

  if args.is_ui_used() {
    let (threads_number, max_waiting_tasks) =
      (args.get_threads_number(), args.get_max_waiting_tasks());
    let (max_actions, max_filled_cells) =
      (args.get_max_actions(), args.get_max_filled_cells());
    if threads_number.is_ok() && max_waiting_tasks.is_ok() &&
      max_actions.is_ok() && max_filled_cells.is_ok() {
        let mut tasks = task::tasksmanager::TasksManager::new(
          threads_number.unwrap(), max_waiting_tasks.unwrap());

        let stdout = std::io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend).unwrap();

        let mut log =
          log::Log::new(max_filled_cells.unwrap(), max_actions.unwrap());

        userinterface::init();
        custom_panic();

        let mut ui = userinterface::UserInterface::new();
        let mut generation_error =
          format!("Error: {:?}", std::io::ErrorKind::Other);

        loop {
          tasks.check_queue();
          match ui.get_error() {
            errors::UserError::NoneError => {
              match ui.get_event() {
                events::UserEvent::Continue => {
                  match ui.get_view() {
                    views::View::Edit => {
                      views::edit::render::render(&mut terminal, &mut log,
                        tasks.get_todo_and_max_tasks());
                      ui.set(views::edit::events::catch_events(&mut log));
                    },
                    views::View::Parameters => {
                      views::parameters::render::render(&mut terminal,
                        &mut log, tasks.get_todo_and_max_tasks());
                      ui.set(views::parameters::events::catch_events(
                        &mut log));
                    },
                  };
                },
                events::UserEvent::SetBrushColor => {
                  views::setbrushcolor::render::render(
                    &mut terminal, &mut log);
                  ui.set_popupview(views::setbrushcolor::events::catch_events(
                    &mut log));
                },
                events::UserEvent::SetGenerationNumber => {
                  views::setgenerationnumber::render::render(&mut terminal,
                    &mut log);
                  ui.set_popupview(
                    views::setgenerationnumber::events::catch_events(&mut log));
                },
                events::UserEvent::SetTextureFormat => {
                  views::settextureformat::render::render(&mut terminal,
                    &mut log);
                  ui.set_popupview(
                    views::settextureformat::events::catch_events(&mut log));
                },
                events::UserEvent::SwitchGridsOrder => {
                  views::switchgridsorder::render::render(&mut terminal,
                    &mut log);
                  ui.set_popupview(
                    views::switchgridsorder::events::catch_events(&mut log));
                },
                events::UserEvent::SwitchGridsError => {
                  views::switchgridserror::render::render(&mut terminal);
                  ui.set_popupview(
                    views::switchgridserror::events::catch_events());
                },
                events::UserEvent::GridName |
                events::UserEvent::RenameGrid => {
                  views::gridname::render::render(&mut terminal, &mut log);
                  ui.set_popupview(views::gridname::events::catch_events(
                    &mut log, ui.get_event()));
                },
                events::UserEvent::GridNumberError => {
                  views::gridnumbererror::render::render(&mut terminal);
                  ui.set_popupview(
                    views::gridnumbererror::events::catch_events());
                },
                events::UserEvent::Generate => {
                  match tasks.new_generation_task(&log) {
                    (Ok(_), thread_is_available) => {
                      if thread_is_available {
                        ui.set_event(events::UserEvent::Continue);
                      } else {
                        ui.set_event(events::UserEvent::UnavailableThread);
                      }
                    },
                    (Err(error), _) => {
                      ui.set_event(events::UserEvent::GenerationError);
                      generation_error = format!("Error: {:?}", error.kind());
                    },
                  }
                },
                events::UserEvent::GenerationError => {
                  views::generationerror::render::render(
                    &mut terminal, &generation_error);
                  ui.set_popupview(
                    views::generationerror::events::catch_events());
                },
                events::UserEvent::LogOverflow => {
                  views::logoverflow::render::render(&mut terminal);
                  ui.set_popupview(views::logoverflow::events::catch_events());
                },
                events::UserEvent::UnavailableThread => {
                  views::unavailablethread::render::render(&mut terminal);
                  ui.set_popupview(
                    views::unavailablethread::events::catch_events());
                },
                events::UserEvent::Exit => {
                  views::exit::render::render(&mut terminal);
                  ui.set_popupview(views::exit::events::catch_events());

                  if let events::UserEvent::Shutdown = ui.get_event() {
                    tasks.start_sender_channel();
                    tasks.compute_todotasks();
                    tasks.spawn_unlocking_thread();
                  }
                },
                events::UserEvent::Shutdown => {
                  if tasks.is_running() {
                    views::shutdown::render::render(&mut terminal,
                      tasks.get_threads_monitor(),
                      tasks.get_done_and_todo_tasks());
                    ui.set_error(views::shutdown::events::catch_events());
                  } else {
                    tasks.shutdown();
                    break;
                  }
                },
              }
            },
            errors::UserError::ResizeCheck => {
              ui.set_error(
                userinterface::UserInterface::check_terminal_size(
                  ui.get_event(), ui.get_view()));
            },
            errors::UserError::ResizeError => {
              views::resizeerror::render::render(&mut terminal);
              ui.set_error(views::resizeerror::events::catch_events());
            },
          }
        }

        userinterface::finish();
    } else {
      if threads_number.is_err() {
        let error = threads_number.unwrap_err();
        println!("Error {:?}: {}", error.kind(), error);
      }
      if max_waiting_tasks.is_err() {
        let error = max_waiting_tasks.unwrap_err();
        println!("Error {:?}: {}", error.kind(), error);
      }
      if max_actions.is_err() {
        let error = max_actions.unwrap_err();
        println!("Error {:?}: {}", error.kind(), error);
      }
      if max_filled_cells.is_err() {
        let error = max_filled_cells.unwrap_err();
        println!("Error {:?}: {}", error.kind(), error);
      }
    }
  } else {
    println!("Type \"liriodendron -h\" or \"liriodendron --help\" to see \
      how to use Liriodendron.");
  }
}
