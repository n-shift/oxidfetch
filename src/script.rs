use mlua::{Error, FromLua, Lua, ToLua, UserData, UserDataFields, Value};

use anyhow::{Context, Result};
use std::fs::File;
use std::io::Read;

use crate::config::{Component, Config, Logo};

impl<'lua> ToLua<'lua> for Component {
    fn to_lua(self, lua: &'lua Lua) -> mlua::Result<Value<'lua>> {
        let component = lua.create_table()?;
        component.set("name", self.name)?;
        component.set("icon", self.icon.unwrap_or("".into()))?;
        component.set("content", self.content)?;

        Ok(Value::Table(component))
    }
}

impl<'lua> FromLua<'lua> for Component {
    fn from_lua(value: Value<'lua>, _: &'lua Lua) -> mlua::Result<Self> {
        if let Value::Table(table) = value {
            let name: String = table.get("name")?;
            let mut icon: Option<String> = Some(table.get("icon")?);
            if icon.clone().unwrap().is_empty() {
                icon = None;
            }
            let content: String = table.get("content")?;

            Ok(Component {
                name,
                icon,
                content,
            })
        } else {
            Err(Error::FromLuaConversionError {
                from: value.type_name(),
                to: "Vec",
                message: Some("expected table".to_string()),
            })
        }
    }
}

impl UserData for Config {
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("logo", |_, this| match &this.logo {
            Logo::Os => Ok(vec!["Os".to_string()]),
            Logo::Custom(val) => {
                let mut value = val.clone();
                let mut out = vec!["Custom".to_string()];
                out.append(&mut value);

                Ok(out)
            }
            Logo::Disabled => Ok(vec!["Disabled".to_string()]),
        });
        fields.add_field_method_set("logo", |_, this, val: Vec<String>| {
            match val[0].as_str() {
                "Os" => this.logo = Logo::Os,
                "Custom" => this.logo = Logo::Custom(val[1..val.len()].to_vec()),
                "Disabled" => this.logo = Logo::Disabled,
                _ => (),
            }

            Ok(())
        });

        fields.add_field_method_get("components", |_, this| Ok(this.components.clone()));
        fields.add_field_method_set("components", |_, this, val: Vec<Component>| {
            this.components = val;

            Ok(())
        });

        fields.add_field_method_get("newline", |_, this| Ok(this.newline));
        fields.add_field_method_set("newline", |_, this, val: bool| {
            this.newline = val;

            Ok(())
        });

        fields.add_field_method_get("spacing", |_, this| Ok(this.spacing));
        fields.add_field_method_set("spacing", |_, this, val: usize| {
            this.spacing = val;

            Ok(())
        });

        fields.add_field_method_get("oneline", |_, this| Ok(this.oneline));
        fields.add_field_method_set("oneline", |_, this, val: bool| {
            this.oneline = val;

            Ok(())
        });
    }
}

pub fn extract_config() -> Result<Config> {
    // get config path
    #[cfg(target_os = "windows")]
    let cfg_path = format!(
        "{}\\.config\\oxidfetch\\config.lua",
        std::env::var("USERPROFILE").unwrap()
    );
    #[cfg(not(target_os = "windows"))]
    let cfg_path = format!(
        "{}/.config/oxidfetch/config.lua",
        std::env::var("HOME").unwrap()
    );

    // execute lua code
    let lua = Lua::new();
    let globals = lua.globals();
    let mut lua_file = File::open(cfg_path).context("failed to open config file")?;
    let mut lua_content = String::new();
    lua_file
        .read_to_string(&mut lua_content)
        .context("failed to read file")?;

    globals.set("cfg", Config::new())?;
    lua.load(&lua_content)
        .set_name("config.lua")
        .context("failed to set name for lua chunk")?
        .exec()
        .context("failed to execute config.lua")?;
    globals
        .get::<_, Config>("cfg")
        .context("failed to get config variable")
}

// TODO: tests
