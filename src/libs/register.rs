use std::fmt::Display;
use uuid::Uuid;
use chrono::prelude::*;

#[derive(Debug)]
pub struct Register {
    pub id: String,
    pub date: NaiveDate,
    pub time: NaiveTime,
    pub state: RegisterState,
    pub row_id: Option<i32>
}

#[derive(Debug, PartialEq)]
pub enum RegisterState {
    Start(),
    Stop(),
    Default()
}

impl RegisterState {
    pub fn from(state: &str) -> RegisterState {
        match state {
            "start" => RegisterState::Start(),
            "stop" => RegisterState::Stop(),
            _ => {
                panic!("Não foi possível convertar a string state para um enum RegisterState")
            }
        }
    }

    pub fn to(state: &RegisterState) -> String {
        match state {
            RegisterState::Start() => String::from("start"),
            RegisterState::Stop() => String::from("stop"),
            _ => {
                panic!("Não foi possível converter um RegisterState para um string")
            }
        }
    }
}

impl Display for RegisterState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Register  {
    pub fn new() -> Self {
        return Self {
            id: String::new(),
            date: NaiveDate::default(),
            time: NaiveTime::default(),
            state: RegisterState::Default(),
            row_id: Some(i32::default())
        }
        
    }

    pub fn new_start_state() -> Self {
        let id: String = Uuid::new_v4().to_string();
        let datetime = Local::now();

        return Self {
            id,
            date: datetime.date_naive(),
            time: datetime.time(),
            state: RegisterState::Start(),
            row_id: None
        }
        
    }

    pub fn new_close_state() -> Self {
        let id: String = Uuid::new_v4().to_string();
        let datetime = Local::now();

        return Self {
            id,
            date: datetime.date_naive(),
            time: datetime.time(),
            state: RegisterState::Stop(),
            row_id: None
        }
        
    }

    pub fn is_start(&self) -> bool {
        *&self.state == RegisterState::Start()
    }

    pub fn is_close(&self) -> bool {
        *&self.state == RegisterState::Stop()
    }
}
