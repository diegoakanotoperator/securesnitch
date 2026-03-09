use tray_icon::{TrayIconBuilder, Icon, menu::{Menu, MenuItem}};

pub fn create_sysbar() -> anyhow::Result<tray_icon::TrayIcon> {
    // In a real app, you would load an actual icon byte array here.
    // For scaffolding, we create a 1x1 invisible icon.
    let rgba = vec![0, 0, 0, 0];
    let icon = Icon::from_rgba(rgba, 1, 1)?;
    
    let tray_menu = Menu::new();
    let show_dashboard = MenuItem::new("Show Dashboard", true, None);
    let quit_item = MenuItem::new("Quit SecureSnitch", true, None);
    
    tray_menu.append(&show_dashboard)?;
    tray_menu.append(&quit_item)?;

    let tray_icon = TrayIconBuilder::new()
        .with_menu(Box::new(tray_menu))
        .with_tooltip("SecureSnitch Firewall")
        .with_icon(icon)
        .build()?;

    Ok(tray_icon)
}