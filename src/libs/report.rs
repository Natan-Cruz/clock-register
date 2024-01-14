

pub mod report {
    use chrono::{Local, Timelike, NaiveTime};
    use crate::libs::register::Register;

    pub fn get_total_hours_worked_on_day_formatted(rows: &Vec<&Register>) -> String {
        let total = get_total_hours_worked_on_day(rows);
        return get_formated_hours_from_seconds(total)
    }

    fn get_total_hours_worked_on_day(rows: &Vec<&Register>) -> i64 {
        if rows.is_empty() {
            return 0
        }

        let rows_lenght = rows.len();
        

        let total = rows.into_iter().enumerate().fold(0, |mut acc, ( index, register )| {
            if index == 0 && register.is_close() {
                acc += register.time.num_seconds_from_midnight() as i64;

                return acc
            }

            let total_seconds = get_seconds_from_naive_time(register.time);


            if index == rows_lenght - 1 && register.is_start() {
                let seconds_now = get_seconds_from_naive_time(Local::now().time());
                acc += seconds_now - total_seconds;
                return acc
            }


            if register.is_start() {
                acc -= total_seconds as i64;
                return acc
            }

            if register.is_close() {
                acc += total_seconds as i64;
                return acc
            }


            return acc
        });


        if total < 0 {
            return total * -1
        }

        return total;
    }

    fn get_seconds_from_naive_time(time: NaiveTime) -> i64 {
        let hours = time.hour();
        let minutes = time.minute();
        let seconds = time.second();

        return ((hours * 60 * 60) + (minutes * 60) + seconds) as i64;
    }

    fn get_formated_hours_from_seconds(seconds: i64) -> String {
        let _seconds = seconds % 60;
        let minutes = (seconds / 60) % 60;
        let hours = (seconds / 60) / 60;
        
        format!("{:0>2}:{:0>2}:{:0>2}", hours, minutes, _seconds)
    }


    
#[cfg(test)]
mod test {
    use chrono::{Local, Duration, TimeZone, NaiveTime};
    use crate::libs::{register::{RegisterState, Register}, report::report::{get_total_hours_worked_on_day, get_seconds_from_naive_time, get_formated_hours_from_seconds}};

    #[test]
    fn it_can_sum_between_open_and_close() {
        let start_time = Local::now().time();
        let end_time = Local::now().checked_add_signed(Duration::hours(1)).unwrap().time();

        let rows = vec![
            Register {time: start_time, state: RegisterState::Start(), ..Register::new()},
            Register {time: end_time, state: RegisterState::Stop(), ..Register::new()},
        ];

        let total = get_total_hours_worked_on_day(&rows.iter().map(|r| r).collect());

        assert_eq!(total, 1 * 60 * 60 )
    }

    #[test]
    fn it_first_day_register_if_close() {
        // terminou às 6 horas da manhã
        let end_time = Local.with_ymd_and_hms(2024, 01, 13, 6, 0, 0).unwrap().time();

        let rows = vec![
            Register {time: end_time, state: RegisterState::Stop(), ..Register::new()},
        ];

        let total = get_total_hours_worked_on_day(&rows.iter().map(|r| r).collect());

        assert_eq!(total, 6 * 60 * 60)
    }


    #[test]
    fn it_first_day_register_if_start() {
        // começou às 6 horas da manhã
        let start_time = Local::now().date_naive().and_hms_opt(6, 0, 0).unwrap().time();

        let rows = vec![
            Register {time: start_time, state: RegisterState::Start(), ..Register::new()},
        ];

        let total = get_total_hours_worked_on_day(&rows.iter().map(|r| r).collect());


        let seconds_start_time = get_seconds_from_naive_time(start_time);
        let seconds_now = get_seconds_from_naive_time(Local::now().time());

        assert_eq!(total, seconds_now - seconds_start_time)
    }

    #[test]
    fn it_multiples_registers() {
        // começou às 6 horas da manhã
        fn get_naive_time_from_hms(h: u32, m: u32, s: u32) -> NaiveTime {
            Local::now().date_naive().and_hms_opt(h, m, s).unwrap().time()
        }

        let rows = vec![
            Register {time: get_naive_time_from_hms(0,0,0), state: RegisterState::Start(), ..Register::new()},
            Register {time: get_naive_time_from_hms(5,0,0), state: RegisterState::Stop(), ..Register::new()},
            Register {time: get_naive_time_from_hms(7,0,0), state: RegisterState::Start(), ..Register::new()},
            Register {time: get_naive_time_from_hms(12,0,0), state: RegisterState::Stop(), ..Register::new()},
            Register {time: get_naive_time_from_hms(20,0,0), state: RegisterState::Start(), ..Register::new()},
            Register {time: get_naive_time_from_hms(22,0,0), state: RegisterState::Stop(), ..Register::new()},
        ];

        let total = get_total_hours_worked_on_day(&rows.iter().map(|r| r).collect());

        assert_eq!(total, 12 * 60 * 60)
    }

    #[test]
    fn it_format_time_from_seconds_works(){
        assert_eq!(get_formated_hours_from_seconds(1 * 60 * 60), "01:00:00");
        assert_eq!(get_formated_hours_from_seconds(12 * 60 * 60), "12:00:00");
    }
}
}

