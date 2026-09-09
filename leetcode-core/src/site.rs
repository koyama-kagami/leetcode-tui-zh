use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

/// A process uses one site so credentials and request caches cannot cross sites.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Site {
    #[default]
    Cn,
    Com,
}

static SITE: OnceLock<Site> = OnceLock::new();

impl Site {
    pub fn origin(self) -> &'static str {
        match self {
            Self::Cn => "https://leetcode.cn",
            Self::Com => "https://leetcode.com",
        }
    }
    pub fn key(self) -> &'static str {
        match self {
            Self::Cn => "cn",
            Self::Com => "com",
        }
    }
}

pub fn current() -> Site {
    SITE.get().copied().unwrap_or_default()
}

pub(crate) fn set(site: Site) -> crate::errors::AppResult<()> {
    if SITE.get().is_some_and(|existing| *existing != site) {
        return Err(crate::errors::LcAppError::SiteAlreadyInitialized);
    }
    SITE.get_or_init(|| site);
    Ok(())
}

pub fn endpoint(path: &str) -> String {
    format!("{}{}", current().origin(), path)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sites_have_separate_origins() {
        assert_eq!(Site::Cn.origin(), "https://leetcode.cn");
        assert_eq!(Site::Com.origin(), "https://leetcode.com");
        assert_eq!(serde_json::from_str::<Site>("\"com\"").unwrap(), Site::Com);
        assert!(serde_json::from_str::<Site>("\"other\"").is_err());
    }
}
