#[cfg(target_os = "linux")]
pub fn drop_privileges() -> anyhow::Result<()> {
    println!("Dropping unnecessary root privileges...");
    
    let mut to_keep = caps::CapsHashSet::new();
    to_keep.insert(caps::Capability::CAP_NET_ADMIN);
    to_keep.insert(caps::Capability::CAP_BPF);
    
    caps::set(None, caps::CapSet::Effective, &to_keep)?;
    caps::set(None, caps::CapSet::Permitted, &to_keep)?;
    
    println!("Privileges dropped. Retaining only CAP_NET_ADMIN and CAP_BPF.");
    Ok(())
}

#[cfg(not(target_os = "linux"))]
pub fn drop_privileges() -> anyhow::Result<()> {
    Ok(())
}