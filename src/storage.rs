
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use dotenv::dotenv;
use mysql::Pool;
use std::env;


pub struct Entry {

  
  pub site: String,
  pub user_name: String,
  pub password: String,
  pub creatoin_date: String,

}






fn main() {

let password = env::var("DB_PASSWORD").expect("DB_PASSWORD must be set");
let connection_string = format!("mysql://rust_app:{}@localhost:3306/password_manager", password);

pub async fn connect_db() -> Result<mysql::Pool, mysql::Error> {
  dotenv().ok();
  let password = env::var("DB_PASSWORD").expect("DB_PASSWORD must be set");
  let connection_string = format!("mysql://rust_app:{}@localhost:3306/password_manager", password);
  
  return Pool::new(connection_string)  ;

}
 
  
}