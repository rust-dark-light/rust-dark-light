use std::time::Duration;

use crate::{Error, Mode};

use ashpd::desktop::settings::ColorScheme as PortalColorScheme;
use ashpd::desktop::settings::Settings as XdgPortalSettings;
use async_std::{future, task};

pub fn detect() -> Result<Mode, Error> {
    task::block_on(async {
        future::timeout(Duration::from_millis(25), get_color_scheme())
            .await
            .map_err(|_| Error::Timeout)?
    })
}

pub(crate) async fn get_color_scheme() -> Result<Mode, Error> {
    let settings = XdgPortalSettings::new().await?;
    let color_scheme = settings.color_scheme().await?;
    Ok(color_scheme.into())
}

impl From<PortalColorScheme> for Mode {
    fn from(value: PortalColorScheme) -> Self {
        match value {
            PortalColorScheme::NoPreference => Mode::Unspecified,
            PortalColorScheme::PreferDark => Mode::Dark,
            PortalColorScheme::PreferLight => Mode::Light,
        }
    }
}
