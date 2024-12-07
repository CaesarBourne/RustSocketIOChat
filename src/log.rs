// log.rs
use log::{error, info};
use std::fs::OpenOptions;
use std::io::Write;

pub fn init_logger() {
    env_logger::init();
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("broadcast.log")
        .unwrap();
    let mut writer = std::io::BufWriter::new(file);
    writer.write_all(b"Broadcast log initialized\n").unwrap();
}

pub fn log_broadcast_message(message: &str, sender_id: &str, recipient_id: &str) {
    info!(
        "Broadcast message from {} to {}: {}",
        sender_id, recipient_id, message
    );
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("broadcast.log")
        .unwrap();
    let mut writer = std::io::BufWriter::new(file);
    writer
        .write_all(
            format!(
                "Broadcast message from {} to {}: {}\n",
                sender_id, recipient_id, message
            )
            .as_bytes(),
        )
        .unwrap();
}
