mod ebpf;
mod nfq;
mod procfs;
mod firewall;
mod litebox;
mod ipc;
mod dns;
mod backend;
mod hashing;
mod rules_engine;
mod dns_proxy;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("SecureSnitch Rust Daemon v1.0.0 started.");
    
    // Start DNS-over-HTTPS Proxy
    tokio::spawn(async move {
        if let Err(e) = dns_proxy::start_dns_proxy("127.0.0.1:5353").await {
            eprintln!("DNS Proxy Error: {}", e);
        }
    });

    // Cross-platform backend initialization
    let backend = backend::get_backend()?;
    backend.initialize()?;

    // Phase 5: Parity (DNS Tracking)
    dns::start_dns_tracking();
    
    // Phase 3: Litebox Enclave & Security
    let enclave = litebox::Enclave::new();
    enclave.protect_memory();
    
    tokio::spawn(async move {
        if let Err(e) = ipc::start_grpc_ipc().await {
            eprintln!("IPC Error: {}", e);
        }
    });
    
    // Linux-Specific Phase 1: eBPF
    #[cfg(target_os = "linux")]
    {
        let default_ebpf_path = "/etc/securesnitch/securesnitch.o";
        match ebpf::load_ebpf_module(default_ebpf_path) {
            Ok(_obj) => println!("eBPF module loaded successfully: {}", default_ebpf_path),
            Err(e) => eprintln!("Warning: Could not load eBPF module (ensure you are root and the file exists): {}", e),
        }

        // Phase 2: NFQ listener
        if let Err(e) = nfq::start_nfq_listener() {
            eprintln!("NFQ listener error: {}", e);
        }
    }

    println!("Process Monitoring and Security Modules ready.");
    
    // Simple keep-alive for the daemon
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
    }
}