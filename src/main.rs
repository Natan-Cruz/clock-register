use linked_hash_map::LinkedHashMap;
use build_html::{self, Html, HtmlContainer, ContainerType, Table, Container};
use chrono::{NaiveDate, Days};
use clap::{Subcommand, Parser};


mod libs;
use libs::{database as db, register::{self, Register}, report::report::get_total_hours_worked_on_day_formatted};


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}


#[derive(Subcommand)]
enum Commands {
    /// Inicia a marcação do tempo
    Start,
    /// Para a marcação do tempo
    Stop,
    /// Gera o relatório de horas
    GenerateReport,
    /// Mostra status atual
    Status
}

fn main() {
    let database = db::Database::connect();

    database.create_database();
    
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Start) => {
            start();
        },
        Some(Commands::Stop) => {
            stop();
        },
        Some(Commands::GenerateReport) => {
            generate_report();
        },
        Some(Commands::Status) => {
            show_status()
        }
        None => {}
    }
}


fn start(){
    let database = db::Database::connect();

    let register_db = db::RegisterDB::new(&database.connection);

    let last_register = register_db.get_last_register();

    if last_register.is_start() {
        return
    }

    let new_register = register::Register::new_start_state();
    register_db.save(new_register);
}

fn stop(){
    let database = db::Database::connect();

    let register_db = db::RegisterDB::new(&database.connection);

    let last_register = register_db.get_last_register();
    if last_register.is_close() {
        return
    }

    let register = register::Register::new_close_state();
    register_db.save(register);
}

fn generate_report(){
    let mut server = server::HttpServer::new();

    server.get("/", Box::new(|_| {
        let mut response = server::HttpResponse::new();
        response.set_body(generate());
        response
    }));

    server.bind("127.0.0.1:8080");
}


fn generate () -> String {
    let database = db::Database::connect();

    let rows: Vec<Register> = db::get_all_registers(&database.connection);
    let result = get_registers_grouped_by_date(&rows);
    let table = teste(result);

    let mut page = build_html::HtmlPage::new()
        .with_title("Relatório de horas trabalhadas")
        .with_header(1, "Relatório de horas trabalhadas")
        .with_container(
            Container::new(ContainerType::Div)
                .with_table(
                    Table::from(table.body)
                    .with_header_row(table.header)
            )

        );

    page.add_style(include_str!("style.css"));

    return page.to_html_string();
}


fn get_registers_grouped_by_date<'a>(rows: &'a Vec<Register>) -> LinkedHashMap<String, Vec<&'a Register>> {
    let start_date = NaiveDate::from_ymd_opt(2024,1,1).unwrap();
    let end_date = NaiveDate::from_ymd_opt(2024,1,31).unwrap();

    let mut current_date = start_date;

    let mut registrations_grouped_by: LinkedHashMap<String, Vec<&Register>> = LinkedHashMap::new();

    while current_date <= end_date  {
        let mut registrations: Vec<&Register> = vec![];

        for row in rows {
            if current_date.eq(&row.date){
                registrations.push(&row)
            }
        }

        registrations.sort_by( |a,b | a.time.cmp(&b.time) );

        registrations_grouped_by.insert(current_date.format("%d/%m/%Y").to_string(), registrations);

        current_date = current_date.checked_add_days(Days::new(1)).unwrap();
    }

    return registrations_grouped_by;
}


struct HtmlTable {
    body: Vec<Vec<String>>,
    header: Vec<String>
}


fn teste(rows: LinkedHashMap<String, Vec<&Register>>) -> HtmlTable {
    let mut body: Vec<Vec<String>> = vec![];
    let mut header: Vec<String> = vec![];

    let first_column_header = String::from("Data");
    let last_column_header = String::from("Horas trabalhadas");

    let mut max_qtd = 0;

    for (_, registers) in &rows {
        let qtd = registers.len();

        if qtd > max_qtd {
            max_qtd = qtd;
        }

    }

    header.push(first_column_header);

    for index in 0..max_qtd {
        if index % 2 == 0 {
            header.push(String::from("Entrada"));
        } else {
            header.push(String::from("Saída")); 
        }
    }

    header.push(last_column_header);

    for (date, registers) in &rows {
        let mut row: Vec<String> = vec![];

        row.push(date.to_string());

        for column_index in 0..max_qtd {
            let register = registers.get(column_index);

            match register {
                Some(register) => {
                    row.push(register.time.format("%H:%M:%S").to_string())
                },
                None => {
                    row.push("-".to_string())
                }
            }
        }

        row.push(get_total_hours_worked_on_day_formatted(registers));

        body.push(row);
    }

    return HtmlTable{
        body, 
        header
    };
}


fn show_status(){ 
    let database = db::Database::connect();

    let register_db = db::RegisterDB::new(&database.connection);

    let register = register_db.get_last_register();
    // let registers = register_db.get_current_month_registers();

    println!("Status: {}", register.state);
    println!("Data: {}", register.date.format("%d/%m/%Y"));
    println!("Hora: {}", register.time.format("%H:%M:%S"));
}