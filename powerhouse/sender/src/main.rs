use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use serde::{Serialize, Deserialize};
use anyhow::{Result, anyhow};
use reqwest::blocking::Client;
use activemq_client::{Consumer, Producer, QueueConnectionFactory};

#[derive(Debug, Serialize, Deserialize)]
struct Payload {
    date: String,
    time: String,
    global_active_power: f32,
    global_reactive_power: f32,
    voltage: f32,
    global_intensity: f32,
    sub_metering_1: f32,
    sub_metering_2: f32,
    sub_metering_3: f32,
}

fn main() -> Result<()> {
    let file = File::open("data.csv")?;
    let reader = BufReader::new(file);

    //let client = Client::new();
    //let endpoint_url = "http://powertation:6777/data";
    let mut client = StompClient::new(ClientOptions::default().broker_url("tcp://activemqbroker:61613"))?;
    client.connect(&[])?;


    let mut numline = 1;
    for line in reader.lines() {
        let line = line?;
        let fields: Vec<&str> = line.split(';').collect();

        let global_active_power = match fields[2].parse::<f32>() {
            Ok(val) => val,
            Err(e) => {
                println!("Error parsing global_active_power for line '{}': {}", line, e);
                continue;
            }
        };

        let global_reactive_power = match fields[3].parse::<f32>() {
            Ok(val) => val,
            Err(e) => {
                println!("Error parsing global_reactive_power for line '{}': {}", line, e);
                continue;
            }
        };

        let voltage = match fields[4].parse::<f32>() {
            Ok(val) => val,
            Err(e) => {
                println!("Error parsing voltage for line '{}': {}", line, e);
                continue;
            }
        };

        let global_intensity = match fields[5].parse::<f32>() {
            Ok(val) => val,
            Err(e) => {
                println!("Error parsing global_intensity for line '{}': {}", line, e);
                continue;
            }
        };

        let sub_metering_1 = match fields[6].parse::<f32>() {
            Ok(val) => val,
            Err(e) => {
                println!("Error parsing sub_metering_1 for line '{}': {}", line, e);
                continue;
            }
        };

        let sub_metering_2 = match fields[7].parse::<f32>() {
            Ok(val) => val,
            Err(e) => {
                println!("Error parsing sub_metering_2 for line '{}': {}", line, e);
                continue;
            }
        };

        let sub_metering_3 = match fields[8].parse::<f32>() {
            Ok(val) => val,
            Err(e) => {
                println!("Error parsing sub_metering_3 for line '{}': {}", line, e);
                continue;
            }
        };

        let payload = Payload {
            date: fields[0].to_string(),
            time: fields[1].to_string(),
            global_active_power,
            global_reactive_power,
            voltage,
            global_intensity,
            sub_metering_1,
            sub_metering_2,
            sub_metering_3,
        };

        let payload_json = serde_json::to_string(&payload)?;
        println!("{}", payload_json);

        
        //match send_payload(&client, &endpoint_url, &payload) {
        match send_payload(&payload) {
            Ok(()) => println!("Line sent successfully"),
            Err(e) => eprintln!("Error sending line : {}, error {}", numline, e),
        };
        numline += 1;
        
    }
    client.disconnect()?;

    Ok(())
}


fn post_to_broker(payload: &Payload) -> Result<()> {
    // Create a connection factory for the ActiveMQ broker
    let connection_factory = QueueConnectionFactory::new("tcp://localhost:61616")
        .context("Failed to create connection factory")?;

    // Create a producer for the queue that we want to send messages to
    let producer = connection_factory
        .create_producer("my-queue")
        .context("Failed to create producer")?;


    // Create a message from the JSON payload
    let message = producer.create_message().context("Failed to create message")?;
    message.set_text(payload);

    // Send the message to the broker
    producer.send(&message).context("Failed to send message to broker")?;

    Ok(())
}





//fn send_payload(client: &Client, endpoint_url: &str, data: &Payload) -> Result<(), Box<dyn Error>> {
    //let response = client.post(endpoint_url).json(data).send()?;
//    let response  = client.send_with_header("/queue/data", payload.as_bytes(), &[Header::ContentType("application/json".into())])?;
//    if response.status().is_success() {
//        Ok(())
//    } else {
//        Err(format!("HTTP error {}: {}", response.status(), response.text()?).into())
//    }
//}
