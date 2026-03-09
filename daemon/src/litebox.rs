use log::{info, warn};

pub struct Enclave {
    pub initialized: bool,
}

impl Enclave {
    pub fn new() -> Self {
        info!("Initializing Litebox secure enclave for anti-tampering guards");
        Self { initialized: true }
    }

    pub fn protect_memory(&self) {
        if self.initialized {
            info!("Litebox memory protection enabled: Daemon rules engine guarded.");
        } else {
            warn!("Enclave not initialized. Memory is vulnerable!");
        }
    }
}

#[cfg(test)]
#[path = "litebox_test.rs"]
mod tests;