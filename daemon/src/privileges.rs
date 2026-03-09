use caps::{CapSet, Capability, CapsHashSet};

pub fn drop_privileges() -> anyhow::Result<()> {
    #[cfg(target_os = "linux")]
    {
        println!("Dropping unnecessary root privileges...");
        
        let mut to_keep = CapsHashSet::new();
        to_keep.insert(Capability::CAP_NET_ADMIN);
        to_keep.insert(Capability::CAP_BPF);
        
        caps::set(None, CapSet::Effective, &to_keep)?;
        caps::set(None, CapSet::Permitted, &to_keep)?;
        
        println!("Privileges dropped. Retaining only CAP_NET_ADMIN and CAP_BPF.");
    }
    Ok(())
}