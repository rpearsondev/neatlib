use gluesql::{prelude::*, core::result::Error};
use std::sync::Mutex;
use array_tool::vec::Join;

use crate::{common::event_stream::event::{Event}};

lazy_static! {
    pub static ref REPOSITORY: Mutex<SqlRepository> =  Mutex::new(SqlRepository::new());
}

pub struct SqlRepository{
    glue: Glue<SharedMemoryStorage>,
    created_tables: Vec<String>
}

impl SqlRepository{
    pub fn new() -> Self{
        let glue = Glue::new(SharedMemoryStorage::new());
        Self { glue, created_tables: Vec::new() }
    }
    pub fn reset(&mut self){
        let glue = Glue::new(SharedMemoryStorage::new());
        self.glue = glue;
        self.created_tables = Vec::new() 
    }
    pub fn insert_event(&mut self, event: &Event){
        let table_name = format!("{:?}", event.event_type);
        let columns = event.additional_properties.clone();
        if !self.created_tables.contains(&table_name){
            let mut column_strings = Vec::new();
            for (name, datatype) in &columns{
                let column_string = format!("{} {}", name, datatype.get_type().unwrap().to_string());
                column_strings.push(column_string);
            }

            let columns_string= column_strings.join(",");

            let create_table = format!("CREATE TABLE {} (generation INT, {})", table_name, columns_string);
            
            let _ = self.glue.execute(create_table).unwrap();
        }
        self.created_tables.push(table_name.clone());

        let mut column_names = Vec::new();
        let mut column_values = Vec::new();

        for (name, value) in &columns{
            column_names.push(name.clone());
            let d_type: DataType = value.get_type().unwrap();
            let literal_value = match d_type {
                DataType::Boolean => format!("{}", String::from(value)),
                DataType::Int8 => format!("{}", String::from(value)),
                DataType::Int16 => format!("{}", String::from(value)),
                DataType::Int32 => format!("{}", String::from(value)),
                DataType::Int => format!("{}", String::from(value)),
                DataType::Int128 => format!("{}", String::from(value)),
                DataType::Uint8 => format!("{}", String::from(value)),
                DataType::Float => format!("{}", String::from(value)),
                DataType::Text => format!("'{}'", String::from(value)),
                DataType::Bytea => format!("{}", String::from(value)),
                DataType::Date => format!("'{}'", String::from(value)),
                DataType::Timestamp => format!("'{}'", String::from(value)),
                DataType::Time => format!("'{}'", String::from(value)),
                DataType::Interval => format!("'{}'", String::from(value)),
                DataType::Uuid => format!("'{}'", String::from(value)),
                DataType::Map => format!("{}", String::from(value)),
                DataType::List => format!("{}", String::from(value)),
                DataType::Decimal => format!("{}", String::from(value)),
            };
            column_values.push(literal_value);
        }

        let insert_statement = format!("INSERT INTO {} (generation, {}) VALUES ({}, {})", table_name, column_names.join(","), event.generation, column_values.join(","));
        
        let _ = self.glue
        .execute(insert_statement)
        .unwrap();
    }
    pub fn query(&mut self, query: &str) -> Result<Vec<Payload>, Error>{
        self.glue
        .execute(query)
    }
}


pub struct SqlRepositoryEventListener;

impl SqlRepositoryEventListener{
    pub fn process(event: &Event){
        let mut repository = REPOSITORY.lock().unwrap();
        repository.insert_event(event);
    }
    pub fn reset(){
        let mut repository = REPOSITORY.lock().unwrap();
        repository.reset();
    }
    pub fn print_tables(){
        println!("--Printing tables--");
        let mut repository = REPOSITORY.lock().unwrap();
        let tables = repository.created_tables.clone();
        let glue = &mut repository.glue;
        for table in tables{
            let output = glue.execute(format!("select * from  {}", table)).unwrap();
            for line in output{
               println!("{:?}", line)
            }
        }
    }
}
