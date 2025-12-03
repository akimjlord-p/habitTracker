use chrono::{NaiveDateTime, Local, Duration};
use super::completion::Completion;
use serde::{Deserialize, Serialize};
use super::utils::HabitData;


#[derive(Deserialize, Serialize)]
pub struct Habit{
    pub name: String,
    pub completions: Vec<Completion>,
    pub created_at: NaiveDateTime,
    pub frequency: Duration
}


impl Habit{
    pub fn new(habit_data: HabitData) -> Self{
        Self{
            name: habit_data.name,
            completions: Vec::new(),
            created_at: Local::now().naive_local(),
            frequency: habit_data.frequency
        }
    }


    pub fn complete(&mut self, note: Option<String>){
        self.completions.push(Completion::new(note));
    }
}