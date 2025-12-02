use super::commands::{Command, HabitPointer};
use crate::service::{analytics, habit::Habit};
use crate::db::storage::Storage;

pub struct CLi{
    storage: Storage
}

impl CLi{
    fn execute_command(command: Command){
        match command{
            Command::New(items) => todo!(),
            Command::Complete(habit_pointer) => todo!(),
            Command::Analytics(habit_pointer) => todo!(),
            Command::Delete(habit_pointer) => todo!(),
            Command::List => todo!(),
            Command::Quit => todo!(),
        }
    }
}