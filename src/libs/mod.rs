
pub mod register {
    use uuid::Uuid;
    use chrono::prelude::*;

    pub struct Register {
        pub id: String,
        pub date: NaiveDate,
        pub time: NaiveTime,
        pub state: String,
        pub row_id: Option<i32>
    }


    impl Register  {
        pub fn new() -> Self {
            return Self {
                id: String::new(),
                date: NaiveDate::default(),
                time: NaiveTime::default(),
                state: String::new(),
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
                state: String::from("start"),
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
                state: String::from("close"),
                row_id: None
            }
            
        }
    
        pub fn is_start(&self) -> bool {
            &self.state == "start"
        }
    
        pub fn is_close(&self) -> bool {
            &self.state == "close"
        }
    }
}

pub mod db {
    use std::{path::{Path, PathBuf}, env::current_exe};
    use chrono::{NaiveDate, Local, Datelike};

    use super::register::Register;

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
                register.state
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
                        register.state = value.to_string();
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
                        register.state = value.to_string();
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
                    register.state = value.to_string();
                }
                (_, None) => {}
            }

            registers.push(register);

            return true
        }).unwrap();

        return registers;
    }
}


pub mod http {
    use std::{net::{TcpListener, TcpStream}, collections::HashMap, io::{BufReader, BufRead, Write}};

    trait Callback {
        fn call(&self, request: TcpStream) -> Result<String, String>;
    }

    pub struct Server {
        // just get
        routes: HashMap<String, fn() -> Result<String, String>>
    }

    impl Server {
        pub fn new() -> Self {
            Self {  
                routes: HashMap::new()
            }            
        }

        pub fn bind(&self, addr: &str) {
            let listener = match TcpListener::bind(&addr) {
                Ok(listener) => {
                    println!("Servidor rodando: {}", addr);
                    listener
                },
                Err(err) => {
                    panic!("Nao foi possível subir o servidor: {}", err)
                }
            };

            for stream in listener.incoming() {
                match stream {
                    Ok(mut stream) => {
                        let mut reader = BufReader::new(stream.try_clone().expect("Erro ao tentar clonar TcpStream"));

                        let mut request = String::new();
                        
                        reader.read_line(&mut request).expect("Erro ao ler requsição");

                        let route_path: Vec<&str> = request.split(" ").collect();
                        let route_path = route_path[1];

                        match self.routes.get(route_path) {
                            Some(callback) => {
                                let response = match callback()  {
                                    Ok(result) => {
                                        format!("HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Lenght: {}\r\n\r\n{}",  result.len(), result)
                                    }
                                    Err(_err)=> {
                                        "HTTP/1.1 400 BAD REQUEST\r\n\r\n400 Bad Request".to_string()
                                    }                       
                                };

                                if let Err(err) = stream.write_all(response.as_bytes()) {
                                    eprintln!("Erro ao gravar a resposta: {}", err)
                                }
                            },
                            None => {
                                println!("Rota não encontrada")
                            }
                        }
                    },
                    Err(e) => eprintln!("Erro ao aceitar a conexão: {}", e),
                }
            }
        }

        pub fn get(&mut self, path: &str, callback: fn() -> Result<String, String>) {
            self.routes.insert(path.to_string(), callback);
        }
    }
}

