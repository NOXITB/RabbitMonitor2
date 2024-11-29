use axum::extract::State;
use monitor::Monitor;
use clap::Parser;
use std::{thread::sleep, time::Duration};
use std::sync::{Arc, Mutex, MutexGuard};
use axum::{routing::get, Router, response::IntoResponse, http::header};
use network_interface::{NetworkInterface, NetworkInterfaceConfig};

pub mod utils;
pub mod monitor;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = String::from("0.0.0.0"))]
    address: String,

    #[arg(short, long, default_value_t = 8088)]
    port: u16,

    #[arg(short, long, default_value_t = 3)]
    cache: u64,

    #[arg(short, long, default_value_t = 1)]
    logger: u8,
}

#[tokio::main]
async fn main() {
    let args: Args = Args::parse();
    let monitor: Arc<Mutex<Monitor>> = Arc::new(Mutex::new(Monitor::new()));
    let cloned: Arc<Mutex<Monitor>> = monitor.clone();

    std::thread::spawn(move || {
        {
            let mut temp: MutexGuard<Monitor> = monitor.lock().unwrap();
            temp.settings.cache = args.cache;
            temp.settings.logger = args.logger;

            // Get all network interfaces excluding docker ones
            if let Ok(interfaces) = NetworkInterface::show() {
                temp.settings.interfaces = interfaces
                    .into_iter()
                    .filter(|iface| !iface.name.contains("docker"))
                    .map(|iface| iface.name)
                    .collect();
            }
        }

        loop {
            {
                let mut temp: MutexGuard<Monitor> = monitor.lock().unwrap();
                temp.refresh_all_interfaces();
            }
            sleep(Duration::from_millis(args.cache * 1000));
        }
    });

    let app: Router<_, _> = Router::new()
        .route("/metrics", get(metrics))
        .with_state(cloned);

    let address: String = args.address + ":" + &args.port.to_string();
    println!("Blstmo Monitor listening on {}", &address);
    axum::Server::bind(&address.parse().unwrap()).serve(app.into_make_service()).await.unwrap();
}

async fn metrics(State(state): State<Arc<Mutex<Monitor>>>) -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "text/plain; version=0.0.4")],
        utils::create_metrics(state)
    )
}