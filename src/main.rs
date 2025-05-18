use serde::{Deserialize, Serialize};
use std::env::args;
use std::net::UdpSocket;
use std::time::Duration;
use sysinfo::{MemoryRefreshKind, Networks, System};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Line {
    line1: String,
    line2: String,
    line3: String,
    line4: String,
}

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    let server_address = args().nth(1).unwrap_or(String::from("192.168.31.70:9090"));
    let mut sys = System::new();
    let mut networks = Networks::new();
    #[cfg(target_os = "windows")]
    let system_str = format!("{} {}", System::name().unwrap_or(String::from("Unknown")), System::os_version().unwrap_or(String::from("Unknown")));

    #[cfg(target_os = "linux")]
    let system_str = format!("{} {}", System::name().unwrap_or(String::from("Unknown")), System::kernel_version().unwrap_or(String::from("Unknown")));

    loop {
        sys.refresh_cpu_usage();
        let cpu_usage = format!("CPU: {:.2}%", sys.global_cpu_usage());

        sys.refresh_memory_specifics(MemoryRefreshKind::nothing().with_ram());
        let ram_free = sys.available_memory() / 1024 / 1024;
        let ram_total = sys.total_memory() / 1024 / 1024;
        let ram_usage = format!("RAM: {}/{}MB", ram_free, ram_total);

        networks.refresh(true);
        let mut all_rx: u64 = 0;
        let mut all_tx: u64 = 0;
        for (_, network) in networks.iter() {
            all_rx += network.received();
            all_tx += network.transmitted();
        }
        let rx_mb = all_rx as f64 / 1024.0 / 1024.0 * 8.0;
        let tx_mb = all_tx as f64 / 1024.0 / 1024.0 * 8.0;
        let network_usage = format!("NET: {:.2}/{:.2}mBps", rx_mb, tx_mb);

        let message = serde_json::to_value(Line {
            line1: cpu_usage,
            line2: ram_usage,
            line3: network_usage,
            line4: system_str.clone(),
        })?
        .to_string();

        socket.send_to(message.as_bytes(), server_address.clone())?;
        println!("SendSuccess: {}", message);
        std::thread::sleep(Duration::from_millis(args().nth(2).unwrap_or(String::from("1000")).parse().unwrap()));
    }
}
