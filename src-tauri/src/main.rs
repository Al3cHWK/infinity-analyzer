use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Clone)]
struct LogStats {
    total: usize,
    info: usize,
    warning: usize,
    error: usize,
    debug: usize,
    frequent_messages: HashMap<String, usize>,
    frequent_errors: Vec<(String, usize)>,
}

#[derive(Debug, Serialize)]
struct LogAnalysisResult {
    stats: LogStats,
    logs: Vec<String>,
}

#[derive(Debug)]
enum LogLevel {
    Info,
    Warning,
    Error,
    Debug,
}

#[tauri::command]
fn analyze_logs(file_path: String) -> Result<LogAnalysisResult, String> {
    let file = File::open(&file_path).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);
    let logs: Vec<String> = reader.lines().filter_map(Result::ok).collect();
    
    let mut stats = LogStats {
        total: 0,
        info: 0,
        warning: 0,
        error: 0,
        debug: 0,
        frequent_messages: HashMap::new(),
        frequent_errors: Vec::new(),
    };
    
    let mut error_counts = HashMap::new();

    for log in &logs {
        stats.total += 1;
        let log_level = get_log_level(&log);
        match log_level {
            LogLevel::Info => stats.info += 1,
            LogLevel::Warning => stats.warning += 1,
            LogLevel::Error => {
                stats.error += 1;
                *error_counts.entry(log.clone()).or_insert(0) += 1;
            },
            LogLevel::Debug => stats.debug += 1,
        }

        *stats.frequent_messages.entry(log.clone()).or_insert(0) += 1;
    }

    // Преобразование HashMap в отсортированный Vec
    let mut sorted_errors: Vec<_> = error_counts.into_iter().collect();
    sorted_errors.sort_by(|a, b| b.1.cmp(&a.1));
    stats.frequent_errors = sorted_errors;

    Ok(LogAnalysisResult {
        stats,
        logs,
    })
}

fn get_log_level(log: &str) -> LogLevel {
    if log.contains("INFO") {
        LogLevel::Info
    } else if log.contains("WARNING") {
        LogLevel::Warning
    } else if log.contains("ERROR") {
        LogLevel::Error
    } else if log.contains("DEBUG") {
        LogLevel::Debug
    } else {
        LogLevel::Info
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![analyze_logs])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
