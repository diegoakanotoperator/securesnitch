#[cfg(target_os = "linux")]
pub struct Firewall {
    #[allow(dead_code)]
    ipt: iptables::IPTables,
}

#[cfg(target_os = "linux")]
impl Firewall {
    pub fn new() -> anyhow::Result<Self> {
        let ipt = iptables::new(false).map_err(|e| anyhow::anyhow!("IPTables error: {}", e))?;
        Ok(Self { ipt })
    }

    #[allow(dead_code)]
    pub fn block_ip(&self, ip: &str) -> anyhow::Result<()> {
        self.ipt.append("filter", "OUTPUT", &format!("-d {} -j DROP", ip))
            .map_err(|e| anyhow::anyhow!("Failed to add rule: {}", e))?;
        Ok(())
    }
}