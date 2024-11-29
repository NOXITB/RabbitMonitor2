use crate::monitor::Monitor;
use std::sync::{Arc, Mutex};

pub fn create_metrics(monitor: Arc<Mutex<Monitor>>) -> String {
    let temp = monitor.lock().unwrap();
    let mut metrics = Vec::new();

    if temp.settings.logger >= 1 {
        // CPU metrics
        for (period, value) in [
            ("1m", temp.processor.min1),
            ("5m", temp.processor.min5),
            ("15m", temp.processor.min15),
        ] {
            metrics.push(format!(
                "# HELP blstmo_cpu_load CPU load average\n\
                # TYPE blstmo_cpu_load gauge\n\
                blstmo_cpu_load{{period=\"{}\"}} {}\n",
                period, value
            ));
        }

        // Memory metrics
        for (type_, value) in [
            ("total", temp.memory.total),
            ("available", temp.memory.available),
            ("used", temp.memory.used),
            ("free", temp.memory.free),
        ] {
            metrics.push(format!(
                "# HELP blstmo_memory_bytes Memory information in bytes\n\
                # TYPE blstmo_memory_bytes gauge\n\
                blstmo_memory_bytes{{type=\"{}\"}} {}\n",
                type_, value
            ));
        }

        // Swap metrics
        for (type_, value) in [
            ("total", temp.swap.total),
            ("used", temp.swap.used),
            ("free", temp.swap.free),
        ] {
            metrics.push(format!(
                "# HELP blstmo_swap_bytes Swap space in bytes\n\
                # TYPE blstmo_swap_bytes gauge\n\
                blstmo_swap_bytes{{type=\"{}\"}} {}\n",
                type_, value
            ));
        }

        // Storage metrics
        for (type_, value) in [
            ("total", temp.storage.total),
            ("used", temp.storage.used),
            ("free", temp.storage.free),
        ] {
            metrics.push(format!(
                "# HELP blstmo_storage_bytes Storage space in bytes\n\
                # TYPE blstmo_storage_bytes gauge\n\
                blstmo_storage_bytes{{type=\"{}\"}} {}\n",
                type_, value
            ));
        }
    }

    // Always included metrics
    metrics.push(format!(
        "# HELP blstmo_cpu_usage_ratio CPU usage as ratio\n\
        # TYPE blstmo_cpu_usage_ratio gauge\n\
        blstmo_cpu_usage_ratio {:.4}\n",
        temp.processor.percent / 100.0
    ));

    metrics.push(format!(
        "# HELP blstmo_memory_usage_ratio Memory usage as ratio\n\
        # TYPE blstmo_memory_usage_ratio gauge\n\
        blstmo_memory_usage_ratio {:.4}\n",
        temp.memory.percent / 100.0
    ));

    metrics.push(format!(
        "# HELP blstmo_swap_usage_ratio Swap usage as ratio\n\
        # TYPE blstmo_swap_usage_ratio gauge\n\
        blstmo_swap_usage_ratio {:.4}\n",
        temp.swap.percent / 100.0
    ));

    metrics.push(format!(
        "# HELP blstmo_storage_usage_ratio Storage usage as ratio\n\
        # TYPE blstmo_storage_usage_ratio gauge\n\
        blstmo_storage_usage_ratio {:.4}\n",
        temp.storage.percent / 100.0
    ));

    for (direction, value) in [
        ("download", temp.network.download),
        ("upload", temp.network.upload),
    ] {
        metrics.push(format!(
            "# HELP blstmo_network_bytes_total Network traffic in bytes\n\
            # TYPE blstmo_network_bytes_total gauge\n\
            blstmo_network_bytes_total{{direction=\"{}\"}} {}\n",
            direction, value
        ));
    }

    metrics.join("")
}

pub fn mega_bits<T: Into<f64>>(bytes: T) -> f64 {
    (bytes.into() / 1048576.0) * 8.0
}