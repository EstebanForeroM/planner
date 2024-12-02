use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl ToString for Day {
    fn to_string(&self) -> String {
        match self {
            Day::Monday => "Monday".to_string(),
            Day::Tuesday => "Tuesday".to_string(),
            Day::Wednesday => "Wednesday".to_string(),
            Day::Thursday => "Thursday".to_string(),
            Day::Friday => "Friday".to_string(),
            Day::Saturday => "Saturday".to_string(),
            Day::Sunday => "Sunday".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Block {
    pub start_hour: u8,
    pub finish_hour: u8,
    pub day: Day
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Schedule {
    pub blocks: Vec<Block>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Class {
    pub class_name: String,
    pub schedules: Vec<Schedule>
}
