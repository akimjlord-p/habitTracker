enum HabitPointer{
    Name(String),
    Number(i64)
}



enum Command {
    New(Vec<String>),
    Complete(HabitPointer),
    Analytics(HabitPointer),
    List,
    Quit,
}


impl Command {
    fn parse(s: &str) -> Result<Self, String>{
        let words: Vec<&str> = s.split_ascii_whitespace().collect();
        if words.len() == 0{
            return Err("Void input".to_string())
        }
        match words[0] {
            "new" => {
                if words.len() != 3 {
                    let n_arguments = words.len() - 1;
                    return Err(format!("Expected 2 arguments, get {}", (words.len() - 1)).to_string())
                }
                Ok(Command::New(vec![words[1].to_string(), words[2].to_string()]))
            }
            "complete" => {
                if words.len() != 2 {
                    let n_arguments = words.len() - 1;
                    return Err(format!("Expected 1 argument, get {}", (words.len() - 1)).to_string())
                }
                let habit: Result<i64, _> = words[1].parse();
                match habit {
                    Ok(habit_number)  => Ok(Command::Complete(HabitPointer::Number(habit_number))),
                    Err(_) => Ok(Command::Complete(HabitPointer::Name((words[1].to_string()))))
                }
            },
            _ => todo!()
        }
    }
}

