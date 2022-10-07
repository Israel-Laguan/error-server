use flexi_logger::{Age, Cleanup, Criterion, Duplicate, FileSpec, Logger, Naming, WriteMode};

pub fn init_logger() -> Result<flexi_logger::LoggerHandle, std::io::Error> {
    let logger = Logger::try_with_str("info") // Write all error, warn, and info messages
        .unwrap()
        .log_to_file(FileSpec::default())
        .rotate(
            // If the program runs long enough,
            Criterion::Age(Age::Day), // - create a new file every day
            Naming::Timestamps,       // - let the rotated files have a timestamp in their name
            Cleanup::KeepLogFiles(7), // - keep at most 7 log files
        )
        .duplicate_to_stderr(Duplicate::Info)
        .write_mode(WriteMode::BufferAndFlush)
        .start()
        .expect("Failed to start logger");
    Ok(logger)
}
