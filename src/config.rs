//! Config structure and conversion between [msgpack](https://msgpack.org/)

use anyhow::{anyhow, Context, Result};
#[allow(unused_imports)]
use std::convert::{TryFrom, TryInto};
use std::fs;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::Path;

/// Config structure
///
/// Logo and vector of [components](Component) provided by user
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub logo: Logo,
    pub components: Vec<Component>,
    /// newline after component
    pub newline: bool,
    /// spacing between logo and text
    pub spacing: usize,
    /// display name and component text on one line
    pub oneline: bool,
}

/// Config logo variants
///
/// Display custom logo, premade or nothing
// TODO: usage
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Logo {
    /// use premade OS logo
    Os,
    /// use provided [String] as logo
    Custom(Vec<String>),
    /// do not use logo
    Disabled,
}

/// oxidfetch component structure
///
/// Component can be whatever uses wants to have; oxidfetch just glues everything into great fetch
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Component {
    /// name of component
    pub name: String,
    /// icon for component; can be omitted
    pub icon: Option<String>,
    /// text inside of component
    pub content: String,
}

impl Config {
    pub fn new() -> Self {
        Self {
            logo: Logo::Disabled,
            components: vec![Component {
                name: "".into(),
                icon: None,
                content: "".into(),
            }],
            newline: true,
            spacing: 1,
            oneline: true,
        }
    }

    pub fn fetch_msgpack() -> Result<Self> {
        #[cfg(target_os = "windows")]
        let cfg_path = format!(
            "{}\\.config\\oxidfetch\\config.mpack",
            std::env::var("USERPROFILE").unwrap()
        );
        #[cfg(not(target_os = "windows"))]
        let cfg_path = format!(
            "{}/.config/oxidfetch/config.mpack",
            std::env::var("HOME").unwrap()
        );

        if Path::new(&cfg_path).exists() {
            let cfg = File::open(cfg_path).context("failed to load config.mpack")?;
            let mut reader = BufReader::new(cfg);
            let mut buffer: MsgPack = Vec::new();
            reader
                .read_to_end(&mut buffer)
                .context("failed to read config.mpack")?;
            buffer
                .try_into()
                .context("failed to convert msgpack into config")
        } else {
            Err(anyhow!("config.mpack not found"))
        }
    }

    pub fn cache(self) -> Result<()> {
        let msgpack: MsgPack = self.try_into()?;

        #[cfg(target_os = "windows")]
        let mut cfg_path = format!(
            "{}\\.config\\oxidfetch\\",
            std::env::var("USERPROFILE").unwrap()
        );
        #[cfg(not(target_os = "windows"))]
        let mut cfg_path = format!("{}/.config/oxidfetch/", std::env::var("HOME").unwrap());

        if !Path::new(&cfg_path).exists() {
            fs::create_dir_all(&cfg_path).context("failed to create ~/.config/oxidfetch/")?;
        }
        cfg_path += "config.mpack";

        if !Path::new(&cfg_path).exists() {
            let mut file = File::create(&cfg_path).context("failed to create config.mpack file")?;

            file.write_all(&msgpack).context("failed to cache config")?;
        }
        Ok(())
    }
}

/// Alias to `[Vec]<[u8]>`
///
/// Used for better readability
type MsgPack = Vec<u8>;

impl TryFrom<MsgPack> for Config {
    /// Required type by trait
    type Error = anyhow::Error;

    /// Convert given [MsgPack] buffer into [Config]
    ///
    /// ```
    /// let buf: MsgPack = vec![
    ///     0x92, 0x81, 0x2, 0xc0, 0x91, 0x93, 0xa2, 0x4f, 0x53, 0xa1, 0x21, 0xa7, 0x53, 0x6f,
    ///     0x6d, 0x65, 0x20, 0x4f, 0x53,
    /// ];
    ///
    /// // using try_into()
    /// let cfg: Config = buf.try_into()?;
    ///
    /// assert_eq!(cfg, Config {
    ///     logo: Logo::disabled,
    ///     components: vec![Component {
    ///         name: "OS".into(),
    ///         icon: "*".into(),
    ///         content: "Some OS".into(),
    ///     }],
    /// })
    ///
    /// // using try_from()
    /// let cfg: Config = Config::try_from(buf)?;
    ///
    /// assert_eq!(cfg, Config {
    ///     logo: Logo::disabled,
    ///     components: vec![Component {
    ///         name: "OS".into(),
    ///         icon: "*".into(),
    ///         content: "Some OS".into(),
    ///     }],
    /// })
    ///
    /// ```
    fn try_from(buf: MsgPack) -> Result<Self> {
        rmp_serde::from_read_ref::<&[u8], Self>(&&buf[..])
            .context("Failed to create config from msgpack")
    }
}

impl TryFrom<Config> for MsgPack {
    type Error = anyhow::Error;

    fn try_from(config: Config) -> Result<Self> {
        rmp_serde::to_vec(&config).context("Failed to create msgpack structure from config")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn msgpack_to_config() -> Result<()> {
        let comp = Component {
            name: "OS".into(),
            icon: Some("!".into()),
            content: "Some OS".into(),
        };

        let expected_cfg = Config {
            logo: Logo::Disabled,
            components: vec![comp],
            newline: true,
            spacing: 1,
            oneline: false,
        };

        let buf: MsgPack = vec![
            0x95, 0x81, 0x2, 0xc0, 0x91, 0x93, 0xa2, 0x4f, 0x53, 0xa1, 0x21, 0xa7, 0x53, 0x6f,
            0x6d, 0x65, 0x20, 0x4f, 0x53, 0xc3, 0x1, 0xc2,
        ];
        let cfg: Config = buf.try_into().unwrap();

        assert_eq!(cfg, expected_cfg);

        Ok(())
    }

    #[test]
    fn msgpack_from_config() -> Result<()> {
        let comp = Component {
            name: "OS".into(),
            icon: Some("!".into()),
            content: "Some OS".into(),
        };

        let cfg = Config {
            logo: Logo::Disabled,
            components: vec![comp],
            newline: true,
            spacing: 1,
            oneline: false,
        };

        let buf: MsgPack = cfg.try_into().unwrap();
        let expected_buf: MsgPack = vec![
            0x95, 0x81, 0x2, 0xc0, 0x91, 0x93, 0xa2, 0x4f, 0x53, 0xa1, 0x21, 0xa7, 0x53, 0x6f,
            0x6d, 0x65, 0x20, 0x4f, 0x53, 0xc3, 0x1, 0xc2,
        ];

        assert_eq!(buf, expected_buf);

        Ok(())
    }
}
