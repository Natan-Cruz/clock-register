
use std::{path::{Path, PathBuf}, env::current_exe};
use chrono::{NaiveDate, Local, Datelike};

use super::register::{Register, RegisterState};

pub struct Database {
    pub connection: sqlite::Connection
}

impl Database {
    pub fn connect() -> Self {
        let mut path: PathBuf;

        if cfg!(debug_assertions) {
            path = Path::new("database.db").into();
        } else {
            path = current_exe().unwrap();
            path.pop();
            path.push("database.db");
        }

        let connection = match sqlite::open(path)  {
            Ok(conn) => conn,
            Err(err) => {
                panic!("Não foi possível conectar no banco de dados: {}", err)
            }
        };

        Self { 
            connection
        }  
    }


    pub fn create_database(&self){
        let query: &str = "
            CREATE TABLE IF NOT EXISTS registers(id TEXT, date TEXT, time TEXT, reg_type TEXT);
        "; 

        self.connection.execute(query).unwrap();
    }
}


pub struct RegisterDB<'a> {
    pub connection: &'a sqlite::Connection
}

impl<'a> RegisterDB<'a> {
    pub fn new(connection: &'a sqlite::Connection) -> Self {
        Self { connection }
    }

    pub fn save(&self, register: Register){
        let query = format!(
            "INSERT INTO registers VALUES ('{}', '{}', '{}', '{}')", 
            register.id, 
            register.date, 
            register.time, 
            RegisterState::to(&register.state)
        );

        self.connection.execute(query).unwrap()
    }

    pub fn get_last_register(&self) -> Register {

        let mut register = Register::new();

        let query = "SELECT rowid, id, date, time, reg_type from registers ORDER BY rowid desc LIMIT 1";

        self.connection.iterate(query, |pairs: &[(&str, Option<&str>)]| {
            let rowid: (&str, Option<&str>) = pairs[0];
            let id = pairs[1];
            let date = pairs[2];
            let time = pairs[3];
            let state = pairs[4];

            match rowid {
                (_, Some(value)) =>{
                    register.row_id = Some(value.parse::<i32>().unwrap());
                }
                (_, None) => {}
            }

            match id {
                (_, Some(value)) =>{
                    register.id = value.to_string();
                }
                (_, None) => {}
            }

            match date {
                (_, Some(value)) =>{
                    register.date = value.parse().unwrap();
                }
                (_, None) => {}
            }

            match time {
                (_, Some(value)) =>{
                    register.time = value.parse().unwrap();
                }
                (_, None) => {}
            }

            match state {
                (_, Some(value)) =>{
                    register.state = RegisterState::from(value)
                }
                (_, None) => {}
            }
            
            return true;
        }).unwrap();

        return register
    }

    pub fn get_current_month_registers(&self) -> Vec<Register> {

        let start = Local::now().with_day(1).unwrap().format("%Y-%m-%d").to_string(); 
        let end = Local::now().with_day(31).unwrap().format("%Y-%m-%d").to_string();

        let query = format!("SELECT * from registers WHERE date BETWEEN date('{}') AND DATE('{}')", start, end);

        let mut registers : Vec<Register> = vec![];

        self.connection.iterate(query, |pairs|{
            let id = pairs[0];
            let date = pairs[1];
            let time = pairs[2];
            let state = pairs[3];


            let mut register = Register::new();
            
            match id {
                (_, Some(value)) =>{
                    register.id = value.to_string();
                }
                (_, None) => {}
            }

            match date {
                (_, Some(value)) =>{
                    register.date = value.parse().unwrap();
                }
                (_, None) => {}
            }

            match time {
                (_, Some(value)) =>{
                    register.time = value.parse().unwrap();
                }
                (_, None) => {}
            }

            match state {
                (_, Some(value)) =>{
                    register.state = RegisterState::from(value)
                }
                (_, None) => {}
            }

            registers.push(register);

            return true
        }).unwrap();

        return registers;
    }
}
fn last_day_of_month(year: i32, month: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(year, month + 1, 1)
        .unwrap_or(NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap())
        .pred_opt()
        .unwrap()
}

pub fn get_all_registers(connection: &sqlite::Connection) -> Vec<Register> {

    let query = "SELECT * from registers";

    let mut registers : Vec<Register> = vec![];

    connection.iterate(query, |pairs|{
        let id = pairs[0];
        let date = pairs[1];
        let time = pairs[2];
        let state = pairs[3];


        let mut register = Register::new();
        
        match id {
            (_, Some(value)) =>{
                register.id = value.to_string();
            }
            (_, None) => {}
        }

        match date {
            (_, Some(value)) =>{
                register.date = value.parse().unwrap();
            }
            (_, None) => {}
        }

        match time {
            (_, Some(value)) =>{
                register.time = value.parse().unwrap();
            }
            (_, None) => {}
        }

        match state {
            (_, Some(value)) =>{
                register.state = RegisterState::from(value)
            }
            (_, None) => {}
        }

        registers.push(register);

        return true
    }).unwrap();

    return registers;
}
