use detect_desktop_environment::DesktopEnvironment;
use std::path::Path;

use anyhow::{Context, Result};
use zbus::blocking::Connection;
use zvariant::Value;

use crate::Mode;

fn get_freedesktop_color_scheme() -> Result<Option<Mode>> {
    let conn = Connection::session()?;
    let reply = conn.call_method(
        Some("org.freedesktop.portal.Desktop"),
        "/org/freedesktop/portal/desktop",
        Some("org.freedesktop.portal.Settings"),
        "Read",
        &("org.freedesktop.appearance", "color-scheme"),
    );
    if let Ok(reply) = &reply {
        let theme = reply.body::<Value>()?;
        let theme = theme
            .downcast::<u32>()
            .with_context(|| "Failed to parse return value from dbus call to org.freedesktop.appearance color-scheme")?;
        match theme {
            1 => Ok(Some(Mode::Dark)),
            2 => Ok(Some(Mode::Light)),
            _ => Ok(None),
        }
    } else {
        return Ok(None);
    }
}

fn check_file(pattern: &str, path: &Path) -> Mode {
    if let Ok(content) = std::fs::read_to_string(path) {
        let theme = content
            .lines()
            .filter(|line| line.contains(pattern))
            .collect::<String>();
        Mode::from(theme.to_lowercase().contains("dark"))
    } else {
        Mode::Light
    }
}

fn check_config_file(pattern: &str, path: &str) -> Mode {
    if let Some(config_dir) = dirs::config_dir() {
        check_file(pattern, &config_dir.join(path))
    } else {
        Mode::Light
    }
}

fn check_dconf(pattern: &str) -> Mode {
    match dconf_rs::get_string(pattern) {
        Ok(theme) => {
            if theme.to_lowercase().contains("dark") {
                Mode::Dark
            } else {
                Mode::Light
            }
        }
        Err(_) => Mode::Light,
    }
}

pub fn detect() -> Mode {
    match get_freedesktop_color_scheme() {
        Ok(mode) => {
            match mode {
                Some(mode) => mode,
                None => match DesktopEnvironment::detect() {
                    DesktopEnvironment::Cinnamon => {
                        check_dconf("/org/cinnamon/desktop/interface/gtk-theme")
                    }
                    DesktopEnvironment::Gnome => check_dconf("/org/gnome/desktop/interface/gtk-theme"),
                    DesktopEnvironment::Kde => check_config_file("Name=", "kdeglobals"),
                    DesktopEnvironment::Mate => check_dconf("/org/mate/desktop/interface/gtk-theme"),
                    DesktopEnvironment::Unity => check_dconf("/org/gnome/desktop/interface/gtk-theme"),
                    DesktopEnvironment::Xfce => check_config_file(
                        "name=\"ThemeName\"",
                        "xfce4/xfconf/xfce-perchannel-xml/xsettings.xml",
                    ),
                    _ => Mode::Light,
                },
            }
        },
        Err(_) => Mode::Light
    }
}
