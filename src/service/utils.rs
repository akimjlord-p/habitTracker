use chrono::{NaiveDateTime, Duration};


pub fn is_in_range(dt: &NaiveDateTime, start: &NaiveDateTime, end: &NaiveDateTime) -> bool {
    dt >= start && dt <= end
}


pub enum HabitPointer{
    Name(String),
    Number(i64)
}

impl HabitPointer{
    pub fn parse(arg: &str) -> Self{
        arg.parse::<i64>().map(Self::Number).unwrap_or_else(|_| HabitPointer::Name(arg.to_string()))
    }
}

pub struct HabitData{
    pub(super) name: String,
    pub(super) frequency: Duration 
}

impl HabitData{
    pub fn parse(name: &str, frequence: &str) -> Result<Self, String>{
        let parts: Vec<&str> = frequence.split(':').collect();
        if parts.len() != 2{
            return Err(format!("Frequency must be hh:dd, got {}", parts.len()))
        }
        let hours: u64 = parts[0].parse()
            .map_err(|_| format!("Invalid hours '{}'", parts[0]))?;
        let days: u64 = parts[1].parse()
            .map_err(|_| format!("Invalid days '{}'", parts[1]))?;

        let sum_in_secs = (days * 24 + hours) * 60 * 60;

        let duration = Duration::seconds(sum_in_secs as i64);
        Ok(Self{
            name: name.to_string(),
            frequency: duration
        })
    }
}