pub trait FirewallBackend {
    fn initialize(&self) -> anyhow::Result<()>;
    fn block_ip(&self, ip: &str) -> anyhow::Result<()>;
}

#[cfg(target_os = "linux")]
mod linux_backend {
    use super::FirewallBackend;
    use crate::firewall::Firewall;

    pub struct LinuxFirewall {
        inner: Firewall,
    }

    impl LinuxFirewall {
        pub fn new() -> anyhow::Result<Self> {
            Ok(Self { inner: Firewall::new()? })
        }
    }

    impl FirewallBackend for LinuxFirewall {
        fn initialize(&self) -> anyhow::Result<()> {
            println!("Linux Firewall Backend Initialized.");
            Ok(())
        }

        fn block_ip(&self, ip: &str) -> anyhow::Result<()> {
            self.inner.block_ip(ip)
        }
    }
}

#[cfg(any(target_os = "macos", target_os = "windows"))]
mod stub_backend {
    use super::FirewallBackend;

    pub struct StubFirewall {}

    impl StubFirewall {
        pub fn new() -> anyhow::Result<Self> {
            Ok(Self {})
        }
    }

    impl FirewallBackend for StubFirewall {
        fn initialize(&self) -> anyhow::Result<()> {
            println!("Warning: Multi-platform backend for this OS is still in experimental/stub mode.");
            Ok(())
        }

        fn block_ip(&self, _ip: &str) -> anyhow::Result<()> {
            Ok(())
        }
    }
}

pub fn get_backend() -> anyhow::Result<Box<dyn FirewallBackend>> {
    #[cfg(target_os = "linux")]
    {
        Ok(Box::new(linux_backend::LinuxFirewall::new()?))
    }
    #[cfg(any(target_os = "macos", target_os = "windows"))]
    {
        Ok(Box::new(stub_backend::StubFirewall::new()?))
    }
}