//use axum::{Router, routing::get};
use tower_http::services::ServeDir;
use table_to_html::HtmlTable;
use std::fs::File;
use std::env;
use postgres::{Client, NoTls};

use chrono::Utc;
use std::string::String as OtherString;

use axum::Router;

use std::path::PathBuf;
use std::io;
use std::path::Path;
use std::io::Write;

#[derive(sqlx::FromRow, serde::Deserialize, serde::Serialize, Debug)]
pub struct NVMeRecord{  
//    #[serde(deserialize_with = "deserialize_field")]       
    pub ts: chrono::DateTime<Utc>,
    pub instance_name: OtherString,
    pub instance_ip: OtherString,
    pub t: OtherString,
    pub sum_bytes: i32,
    pub sum_lat: f32,
    pub count_t: i32
}

fn get_current_dir() -> Result<PathBuf, io::Error> {
    let curr_dir = env::current_dir();
    Ok(curr_dir?) // If all operations succeed, return Ok(contents)
}

fn check_database() -> Result<bool, postgres::Error> {
    let mut client = Client::connect("postgresql://postgres:postgres@localhost:5432/postgres", NoTls)?;
  
    let rows = client.query("SELECT * FROM pg_catalog.pg_tables 
    WHERE schemaname != 'pg_catalog' AND schemaname != 'information_schema'", 
    &[])?;
    let mut _flag = false;
    for r in rows {
        let value1: OtherString = r.get(0);
        let value2: OtherString = r.get(1);
        if value1 == "metrics_db" && value2 == "nvme_metrics"{ 
            _flag = true;      
        }
    }
    Ok(_flag)
}

fn create_and_save_hrml_table() -> Result<(), Box<dyn std::error::Error>> {
let mut client = Client::connect("postgresql://postgres:postgres@localhost:5432/postgres", NoTls)?;

let mut _data: Vec<Vec<String>> = Vec::new();
let rows = client.query("SELECT * FROM metrics_db.nvme_metrics", &[]).unwrap();
for r in rows {
    let ts: chrono::DateTime<Utc> = r.get(0);
    let ts_str = ts.to_string();
    let instance_name: &str = r.get(1);
    let in_str = instance_name.to_string();
    let instance_ip: &str = r.get(2);
    let inip_str  = instance_ip.to_string();
    let t: &str = r.get(3);
    let t_str = t.to_string();
    let sum_bytes: i32 = r.get(4);
    let sum_bytes_str = sum_bytes.to_string();
    let sum_lat: f32 = r.get(5);
    let sum_lat_str = sum_lat.to_string();
    let count_t: i32 = r.get(6);
    let count_t_str = count_t.to_string();
    println!("{:?} {:?} {:?} {:?} {:?} {:?} {:?}", ts, instance_name, instance_ip,
     t, sum_bytes, sum_lat, count_t); 
    let mut v: Vec<String> = Vec::new();
    v.push(ts_str);
    v.push(in_str);
    v.push(inip_str);
    v.push(t_str);
    v.push(sum_bytes_str);
    v.push(sum_lat_str);
    v.push(count_t_str);
    _data.push(v);

  }
  let mut table = HtmlTable::new(_data);
  table.set_border(1); // Set a border for example

  // 2. Convert the HtmlTable to a String
  let html_string = table.to_string(); //


  let curr_dir = get_current_dir().unwrap();

  println!("curr_dir: {:?}", curr_dir);
  let path: PathBuf = Path::new("/").join(&curr_dir).join(PathBuf::from("public/index.html"));
  println!("path: {:?}", path.display());
  let mut file = File::create(&path)?; // Creates or truncates the file
  file.write_all(html_string.as_bytes())?; // Writes the string as bytes

  println!("HTML table saved to {}", path.display());
//  print_table();
  Ok(())
}



fn main() {
    // --- Synchronous code goes here ---
    println!("Performing some synchronous setup before Axum starts.");
//    let my_data = "Hello from sync setup!".to_string(); 
    let _ = check_database();
    let _ = create_and_save_hrml_table();

    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async {
        // --- Axum server setup and start ---
    //    let dir = "public"; // Replace with your desired directory

        // Create the ServeDir service
    //    let serve_dir = ServeDir::new(dir);
        let app = Router::new()
        // Serve files from the "assets" directory for any route not explicitly defined
        .fallback_service(ServeDir::new("public"));
        let addr = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

        // Start the Axum server
        axum::serve(addr, app).await.unwrap();

    });
}




