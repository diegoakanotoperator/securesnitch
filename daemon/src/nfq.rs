#[cfg(target_os = "linux")]
pub fn start_nfq_listener() -> anyhow::Result<()> {
    log::info!("Starting NFQUEUE listener on queue 0");
    // Stub implementation for NFQUEUE.
    Ok(())
}

#[cfg(not(target_os = "linux"))]
pub fn start_nfq_listener() -> anyhow::Result<()> {
    Ok(())
}