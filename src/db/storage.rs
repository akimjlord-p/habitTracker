use crate::service::habit::Habit;
use serde::{Deserialize, Serialize};
use std::{error::Error, fs};
use crate::service::utils::HabitPointer;

#[derive(Deserialize, Serialize)]
pub struct Storage{
    habits: Vec<Habit>
}


impl Storage{
    pub fn generate() -> Self{
        let file = fs::read_to_string("storage.json");
        match file {
            Ok(file) => match serde_json::from_str(&file) {
                Ok(storage) => storage,
                Err(error) => panic!("{error}")
            },
            Err(_) => {
                Self{
                    habits: Vec::new()
                }
            }
        }
    }


    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        let json = serde_json::to_string_pretty(&self)?;
        fs::write("storage.json", json)?;
        Ok(())
    }

    pub fn add_habit(&mut self, habit: Habit) -> Result<(), Box<dyn Error>>{
        if let Some(_) = self.habits.iter().find(|h| h.name == habit.name){
            return Err(format!("Habit '{}' already exists", habit.name).into())
        }
        self.habits.push(habit);
        self.save()?;
        Ok(())
    }

    pub fn delete_habit(&mut self, habit_pointer: HabitPointer) -> Result<usize, Box<dyn Error>>{
        match habit_pointer {
            HabitPointer::Name(name) => self.delete_by_name(name),
            HabitPointer::Number(idx) => self.delete_by_idx(idx),
        }
    }

    fn delete_by_name(&mut self, habit_name: String) -> Result<usize, Box<dyn Error>>{
        if let None = self.habits.iter().find(|h| h.name == habit_name){
            return Err(format!("Incorrect habit name '{}' ", habit_name).into())
        }
        let start_len =self.habits.len();
        self.habits.retain(|h| h.name != habit_name);
        self.save()?;
        Ok(start_len - self.habits.len())
    }

    fn delete_by_idx(&mut self, idx: i64) -> Result<usize, Box<dyn Error>>{
        if self.habits.get(idx as usize).is_none() {
            return Err("Index out of bounds".into());
        }        
        let start_len =self.habits.len();
        self.habits.remove(idx as usize);
        self.save()?;
        Ok(start_len - self.habits.len())
    }

    pub fn get_habits(&self) -> &[Habit]{
        &self.habits
    }

    pub fn get_mut_habit(&mut self, habit_pointer: HabitPointer) -> Result<&mut Habit, Box<dyn Error>>{
        match habit_pointer {
            HabitPointer::Name(name) => self.get_mut_habit_by_name(&name),
            HabitPointer::Number(idx) => self.get_mut_habit_by_idx(idx),
        }

    }

    fn get_mut_habit_by_name(&mut self, habit_name: &str) -> Result<&mut Habit, Box<dyn Error>>{

        self.habits.iter_mut().find(|h| h.name == habit_name)
        .ok_or_else(|| format!("Incorrect habit name '{}' ", habit_name).into())
    }

    fn get_mut_habit_by_idx(&mut self, idx: i64) -> Result<&mut Habit, Box<dyn Error>>{
        self.habits.get_mut(idx as usize).ok_or_else(||"Index out of bounds".into())
    }
}