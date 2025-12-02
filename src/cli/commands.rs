pub enum HabitPointer{
    Name(String),
    Number(i64)
}

impl HabitPointer{
    fn parse(arg: &str) -> Self{
        arg.parse::<i64>().map(Self::Number).unwrap_or_else(|_| HabitPointer::Name(arg.to_string()))
    }
}



pub enum Command {
    New(Vec<String>),
    Complete(HabitPointer),
    Analytics(HabitPointer),
    Delete(HabitPointer),
    List,
    Quit
}


impl Command {
    pub fn parse(s: &str) -> Result<Self, String>{
        let words: Vec<&str> = s.split_ascii_whitespace().collect();
        if words.len() == 0{
            return Err("Void input".to_string())
        }
        match words[0] {
            "new" => {
                check_n_args(&words, 3)?;
                Ok(Command::New(vec![words[1].to_string(), words[2].to_string()]))
            }
            "complete" => {
                check_n_args(&words, 2)?;
                let habit = HabitPointer::parse(&words[1]);
                Ok(Command::Complete(habit))
            },
            "delete" => {
                check_n_args(&words, 2)?;
                let habit = HabitPointer::parse(&words[1]);
                Ok(Command::Delete(habit))
            },
            "analytics" => {
                check_n_args(&words, 2)?;
                let habit = HabitPointer::parse(&words[1]);
                Ok(Command::Analytics(habit))
            },
            "list" => {
                Ok(Command::List)
            }
            "quit" => {
                Ok(Command::Quit)
            }
            _ => Err("Unexpected command".to_string())
        }
    }
}

fn check_n_args(words: &Vec<&str>, n: i64) -> Result<(), String> {
    if words.len() != n as usize{
        return Err(format!("Expected {} args, get {}", n-1, (words.len() - 1)).to_string())
    }
    else{
        Ok(())
    }
}