//! Config structure and conversion between [msgpack](https://msgpack.org/)

use anyhow::{Context, Result};
#[allow(unused_imports)]
use std::convert::{TryFrom, TryInto};

/// Config structure
///
/// Logo and vector of [components](Component) provided by user
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub logo: Logo,
    pub components: Vec<Component>,
}

/// Config logo variants
///
/// Display custom logo, premade or nothing
// TODO: usage
#[derive(Debug, PartialEq, Serialize, Deserialize)]
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
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Component {
    /// name of component
    pub name: String,
    /// icon for component; can be omitted
    pub icon: Option<String>,
    /// text inside of component
    pub content: String,
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
        };

        let buf: MsgPack = vec![
            0x92, 0x81, 0x2, 0xc0, 0x91, 0x93, 0xa2, 0x4f, 0x53, 0xa1, 0x21, 0xa7, 0x53, 0x6f,
            0x6d, 0x65, 0x20, 0x4f, 0x53,
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
        };

        let buf: MsgPack = cfg.try_into().unwrap();
        let expected_buf: MsgPack = vec![
            0x92, 0x81, 0x2, 0xc0, 0x91, 0x93, 0xa2, 0x4f, 0x53, 0xa1, 0x21, 0xa7, 0x53, 0x6f,
            0x6d, 0x65, 0x20, 0x4f, 0x53,
        ];

        assert_eq!(buf, expected_buf);

        Ok(())
    }
}
