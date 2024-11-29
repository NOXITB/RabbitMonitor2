use crate::monitor::Monitor;
use std::sync::{Arc, Mutex};
use std::fmt::Display;

pub fn mega_bits<T: Into<f64>>(bytes: T) -> f64 {
    (bytes.into() / 1048576.0) * 8.0
}

fn create_metric(name: impl Display, help: impl Display, value: impl Display, labels: Option<&str>) -> String {
    let metric_name = format!("blstmo_{}", name);
    let label_str = labels.unwrap_or("");
    format!("# HELP {} {}\n# TYPE {} gauge\n{}{} {}\n",
        metric_name, help, metric_name, metric_name, label_str, value)
}

pub fn create_metrics(monitor: Arc<Mutex<Monitor>>) -> String {
    let mut metrics = String::new();
    let temp = monitor.lock().unwrap();

    if temp.settings.logger >= 1 {
        // CPU metrics
        metrics.push_str(&create_metric("cpu_load", "CPU load average", temp.processor.min1, Some("{period=\"1m\"}")));
        metrics.push_str(&create_metric("cpu_load", "CPU load average", temp.processor.min5, Some("{period=\"5m\"}")));
        metrics.push_str(&create_metric("cpu_load", "CPU load average", temp.processor.min15, Some("{period=\"15m\"}")));

        // Memory metrics
        metrics.push_str(&create_metric("memory_bytes", "Memory information in bytes", temp.memory.total, Some("{type=\"total\"}")));
        metrics.push_str(&create_metric("memory_bytes", "Memory information in bytes", temp.memory.available, Some("{type=\"available\"}")));
        metrics.push_str(&create_metric("memory_bytes", "Memory information in bytes", temp.memory.used, Some("{type=\"used\"}")));
        metrics.push_str(&create_metric("memory_bytes", "Memory information in bytes", temp.memory.free, Some("{type=\"free\"}")));

        // Swap metrics
        metrics.push_str(&create_metric("swap_bytes", "Swap space in bytes", temp.swap.total, Some("{type=\"total\"}")));
        metrics.push_str(&create_metric("swap_bytes", "Swap space in bytes", temp.swap.used, Some("{type=\"used\"}")));
        metrics.push_str(&create_metric("swap_bytes", "Swap space in bytes", temp.swap.free, Some("{type=\"free\"}")));

        // Storage metrics
        metrics.push_str(&create_metric("storage_bytes", "Storage space in bytes", temp.storage.total, Some("{type=\"total\"}")));
        metrics.push_str(&create_metric("storage_bytes", "Storage space in bytes", temp.storage.used, Some("{type=\"used\"}")));
        metrics.push_str(&create_metric("storage_bytes", "Storage space in bytes", temp.storage.free, Some("{type=\"free\"}")));
    }

    // Always included metrics
    metrics.push_str(&create_metric("cpu_usage_ratio", "CPU usage as ratio", format!("{:.4}", temp.processor.percent / 100.0), None));
    metrics.push_str(&create_metric("memory_usage_ratio", "Memory usage as ratio", format!("{:.4}", temp.memory.percent / 100.0), None));
    metrics.push_str(&create_metric("swap_usage_ratio", "Swap usage as ratio", format!("{:.4}", temp.swap.percent / 100.0), None));
    metrics.push_str(&create_metric("storage_usage_ratio", "Storage usage as ratio", format!("{:.4}", temp.storage.percent / 100.0), None));
    metrics.push_str(&create_metric("network_bytes_total", "Network traffic in bytes", temp.network.download, Some("{direction=\"download\"}")));
    metrics.push_str(&create_metric("network_bytes_total", "Network traffic in bytes", temp.network.upload, Some("{direction=\"upload\"}")));

    metrics
}