//use chrono::Utc;
use std::fs::File;
use std::fs;
use std::error::Error;
use postgres::{Client, NoTls};
use std::io::ErrorKind;
use serde_json::Value;
use chrono::{DateTime, Utc};
use std::string::String as OtherString;
use std::env;
use std::path::PathBuf;
use std::io;
use std::path::Path;

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

fn create_database() -> Result<bool, postgres::Error> {
    let mut _flag : bool = false;
    let mut client = Client::connect("postgresql://postgres:postgres@localhost:5432/postgres", NoTls)?;
    println!("Create ALL From scratch"); 
//                        ts TIMESTAMP WITH TIME ZONE,
    client.batch_execute("CREATE SCHEMA IF NOT EXISTS metrics_db;")?;  
    println!("After Schema");
    match client.batch_execute("CREATE TABLE IF NOT EXISTS metrics_db.nvme_metrics 
                    (

                        ts TIMESTAMP WITH TIME ZONE,
                        instance_name VARCHAR(10),
                        instance_IP VARCHAR(15),
                        t VARCHAR(3),
                        sum_bytes INT,
                        sum_lat REAL,
                        count_t INT);",
                    ) 
{
        Ok(_rows_affected) => println!("Table Created."),
        Err(e) => eprintln!("Error Table Creation: {}", e),   
} 

    _flag = true;
    Ok(_flag)
}

fn get_file_content(file_path : PathBuf) -> Result<OtherString, Box<dyn Error>> {
    let message: OtherString = fs::read_to_string(&file_path)?;
//    println!("{}", file_path);
//    println!("{}", message);
    Ok(message)
 //   Ok(())
}

fn delete_file(file_path : PathBuf) -> std::io::Result<()> {
    fs::remove_file(&file_path)?;
    Ok(())
}

fn get_current_dir() -> Result<PathBuf, io::Error> {
    let curr_dir = env::current_dir();
    Ok(curr_dir?) // If all operations succeed, return Ok(contents)
}
/*
fn method_1(root: impl AsRef<Path>) -> PathBuf {
  let path = root.as_ref().join("directory_a").join("file_b.whatever");
  path
}
(Path::new("/etc").join("/bin/sh"), PathBuf::from("/bin/sh"));*/
fn main() -> Result<(), postgres::Error> {
   let _flag : bool = check_database().unwrap();
    println!("Content: {:?}", _flag);
    let mut client = Client::connect("postgresql://postgres:postgres@localhost:5432/postgres", NoTls)?;
    let fn_array: [&str; 4] = ["mytest1/result_R_Server1.json","mytest2/result_R_Server2.json",
                                "mytest1/result_W_Server1.json","mytest2/result_W_Server2.json"];

    if !_flag {
        let _flag = create_database();
        println!("Database Created: {:?}", _flag);
    }
    println!("Insert JSON values"); 
    for x in fn_array {

        println!("File: {:?}", x);

        let curr_dir = get_current_dir().unwrap();

        println!("curr_dir: {:?}", curr_dir);

        let path_full: PathBuf = Path::new("/").join(&curr_dir).join(PathBuf::from(&x));
        println!("path_full: {:?}", path_full);

//        let file = File::open(&x);
        let file = File::open(&path_full);

        match file {
            Ok(_f) => {
                println!("File opened successfully!");
                
//                let mut _cont =  get_file_content (&x).unwrap();
                let mut _cont =  get_file_content (path_full.clone()).unwrap();
                println!("Content: {:?}", _cont);

                if _cont.contains("instance_name") {


                    let s = _cont.replace(&['\n','\t','[',']'][..], "");
                    println!("s: {:?}", s);
                    match serde_json::from_str::<Value>(&s) {
                        Ok(parsed_json) => {
                            println!("parsed_json: {:?}", parsed_json);
                            let ts_str = parsed_json.get("ts").and_then(|v| v.as_str()).unwrap();
                            println!("ts_str: {:?}", ts_str); 
                            let ts_str_1 = ts_str.replace('+', " +");
                            println!("ts_str_1 {:?}", ts_str_1); 
                            let ts_str_2 = ts_str_1 + ":00";

                            let format = "%Y-%m-%d %H:%M:%S.%f %z"; // %z represents the timezone offset

                            let datetime_fixed_offset = DateTime::parse_from_str(&ts_str_2, format)
                                .expect("Failed to parse datetime with offset");

                            println!("Parsed DateTime with FixedOffset: {}", datetime_fixed_offset);

                            // You can then convert this to UTC or other timezones
                            let datetime_utc = datetime_fixed_offset.with_timezone(&Utc);
                            println!("Converted to UTC: {}", datetime_utc);


                            let instance_name = parsed_json.get("instance_name").and_then(|v| v.as_str()).unwrap();
                            println!("instance_name: {:?}", instance_name); 
                            let instance_ip = parsed_json.get("instance_ip").and_then(|v| v.as_str()).unwrap();
                            println!("instance_ip: {:?}", instance_ip); 
                            let t = parsed_json.get("t").and_then(|v| v.as_str()).unwrap();
                            println!("t: {:?}", t); 
//                        v["age"].as_i64()
                            let sum_bytes = parsed_json.get("sum(BYTES)").and_then(|v| v.as_i64()).unwrap();

                            println!("sum_bytes: {:?}", sum_bytes); 
                            let sum_lat = parsed_json.get("sum(LAT)").and_then(|v| v.as_f64()).unwrap();
                            println!("sum_lat: {:?}", sum_lat); 
                            let count_t = parsed_json.get("count(T)").and_then(|v| v.as_i64()).unwrap();
                            println!("count_t: {:?}", count_t);

                            let sum_bytes_i32 = sum_bytes as i32;
                            let sum_lat_f32 = sum_lat as f32;
                            let count_t_i32 = count_t as i32;

                            let p = NVMeRecord {
                                ts: datetime_utc,
                                instance_name : instance_name.to_string(),
                                instance_ip : instance_ip.to_string(),
                                t : t.to_string(),
                                sum_bytes : sum_bytes_i32,
                                sum_lat : sum_lat_f32,
                                count_t : count_t_i32,
                            };


                    match client.execute("INSERT INTO metrics_db.nvme_metrics(ts, instance_name, instance_ip, t,
                             sum_bytes, sum_lat, count_t) VALUES($1,$2,$3,$4,$5,$6,$7)",
                            &[&p.ts,&p.instance_name,&p.instance_ip,&p.t,&p.sum_bytes,&p.sum_lat,&p.count_t])

                                {
                                    Ok(rows_affected) => {
                                    println!("{} row(s) inserted successfully", rows_affected);
                                    let _ = delete_file(path_full);
                                  }
                                  Err(e) => {
                                    eprintln!("Error inserting data: {}", e);
                                    return Err(e); // Propagate the error if desired
                                  }
                              }

                            },

                        Err(e) => {
                            eprintln!("Error deserializing JSON: {}", e); // Print the error to stderr
                        }
                    }

                }

            },
            Err(error) => match error.kind() {
                ErrorKind::NotFound => println!("File not found!"),
                ErrorKind   ::PermissionDenied => println!("Permission denied!"),
                other_error => println!("Other error: {:?}", other_error),
            },
        }
        continue;

    }

    Ok(())
}
   
