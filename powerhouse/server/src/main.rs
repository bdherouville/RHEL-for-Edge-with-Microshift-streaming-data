//use tokio::net::{TcpStream};
use async_std::stream::StreamExt;
//use async_std::net::TcpListener;
use async_std::io::ReadExt;
use serde::{Deserialize};
use serde_json::{json};
use chrono::NaiveDateTime;
use chrono::NaiveDate;
use chrono::NaiveTime;
use chrono::DateTime;
use chrono::Utc;
//use std::time::Instant;

#[derive(Deserialize)]
struct MyData {
    date: String,
    time: String,
    global_active_power: f64,
    global_reactive_power: f64,
    voltage: f64,
    global_intensity: f64,
    sub_metering_1: f64,
    sub_metering_2: f64,
    sub_metering_3: f64,
}


#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:6777";
    //let listener = TcpListener::bind(addr).await.unwrap();
    //let mut listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    let listener = async_std::net::TcpListener::bind("0.0.0.0:6777").await.unwrap();

    println!("Listening on: {}", addr);

    let mut incoming = listener.incoming();
    //
    while let Some(conn_result) = incoming.next().await {
        match conn_result {
            Ok(mut socket) => {
                let mut buf = [0; 1024];
                let n = socket.read(&mut buf).await.unwrap();
                let req_str = String::from_utf8_lossy(&buf[..n]);
                    let _response = if req_str.starts_with("POST /data HTTP/1.1") {
                    let json_str = req_str.splitn(2, "\r\n\r\n").nth(1).unwrap_or("");
                let json_str = json_str.trim_end_matches('\0');
        let data: Result<MyData, _> = serde_json::from_str(json_str);
        match data {
           Ok(my_data) => {

                let date = NaiveDate::parse_from_str(&my_data.date, "%d/%m/%Y").unwrap();
                let time = NaiveTime::parse_from_str(&my_data.time, "%H:%M:%S").unwrap();
                let datetime = NaiveDateTime::new(date, time);
                let timestamp = DateTime::<Utc>::from_utc(datetime, Utc).timestamp();

                let global_active_power = my_data.global_active_power;
                let global_reactive_power = my_data.global_reactive_power;
                let voltage = my_data.voltage;
                let global_intensity = my_data.global_intensity;
                let sub_metering_1 = my_data.sub_metering_1;
                let sub_metering_2 = my_data.sub_metering_2;
                let sub_metering_3 = my_data.sub_metering_3;
               
                let result = json!({
                    "timestamp:": timestamp,
                    "global_active_power:": global_active_power,
                    "global_reactive_power:": global_reactive_power,
                    "voltage:": voltage,
                    "global_intensity:": global_intensity,
                    "sub_metering_1:": sub_metering_1,
                    "sub_metering_2:": sub_metering_2,
                    "sub_metering_3:": sub_metering_3,
                });

                let response_data = serde_json::to_string(&result).unwrap();
                format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
                    response_data.len(),
                    response_data
                )
            }        
            Err(e) => {
                format!("HTTP/1.1 400 Bad Request\r\n\r\nError parsing JSON: {}", e)
           }
        }

                    
                    } else {
                    "HTTP/1.1 404 NOT FOUND\r\n\r\n".to_owned()
                    };

                println!("Received request: {}", req_str);
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
}
