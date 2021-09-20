//! Rendering config into text
use crate::config::{Config, Logo};
use regex::Regex;

/// Render colors inside given string
///
/// Takes [String] as input and replaces \[color\] with ansi escape code. To reset colors use
/// \[_\].
fn colorize(text: String) -> String {
    let pattern_general = Regex::new(r"((?:\\\\)*\[.*?(?:\\\\)*\])?([^\[]*)").unwrap();
    let pattern_color = Regex::new(r"(?:\\\\)*\[(.*?)(?:\\\\)*\]").unwrap();
    let mut display: Vec<String> = Vec::new();
    for found in pattern_general.captures_iter(&text) {
        if let Some(color) = found.get(1) {
            display.push(color.as_str().into());
        }
        display.push(found[2].into());
    }

    if display.is_empty() {
        display.push(text);
    }

    let mut colored: Vec<String> = Vec::new();
    for item in display {
        if pattern_color.is_match(&item) {
            let captures = pattern_color.captures_iter(&item);
            let index = {
                if colored.len() != 0 {
                    Some(colored.len() - 1)
                } else {
                    None
                }
            };

            if index != None {
                let last_item = colored.get_mut(index.unwrap()).unwrap();
                for found in captures {
                    match &found[1] {
                        "black" => *last_item += "\x1b[30m",
                        "red" => *last_item += "\x1b[31m",
                        "green" => *last_item += "\x1b[32m",
                        "yellow" => *last_item += "\x1b[33m",
                        "blue" => *last_item += "\x1b[34m",
                        "magenta" => *last_item += "\x1b[35m",
                        "cyan" => *last_item += "\x1b[36m",
                        "white" => *last_item += "\x1b[37m",
                        "_" => *last_item += "\x1b[0m",
                        &_ => (),
                    };
                }
            } else {
                for found in captures {
                    match &found[1] {
                        "black" => colored.push("\x1b[30m".into()),
                        "red" => colored.push("\x1b[31m".into()),
                        "green" => colored.push("\x1b[32m".into()),
                        "yellow" => colored.push("\x1b[33m".into()),
                        "blue" => colored.push("\x1b[34m".into()),
                        "magenta" => colored.push("\x1b[35m".into()),
                        "cyan" => colored.push("\x1b[36m".into()),
                        "white" => colored.push("\x1b[37m".into()),
                        "_" => colored.push("\x1b[0m".into()),
                        &_ => (),
                    };
                }
            }
        } else {
            colored.push(item.into());
        }
    }
    colored.join("")
}

/// [config::Config](crate::config::Config) to `[Vec]<[String]>`
///
/// Takes passed [config::Config](crate::config::Config),
/// merges logo (vector of logo lines) and components:
/// ```txt
/// logoline1 {component1.icon}{component1.name}:
/// logoline2 {component1.content}
/// logoline3
/// {component2.icon}{component2.name}:
/// {component2.content}
/// ```
fn render(cfg: Config) -> Vec<String> {
    // logo
    let mut colorless_logo: Vec<String> = Vec::new();
    let mut logo: Vec<String> = Vec::new();

    // write logo to variable
    match cfg.logo {
        Logo::Os => {
            unimplemented!();
        }
        Logo::Custom(provided_logo) => {
            colorless_logo = provided_logo;
        }
        Logo::Disabled => {}
    }

    if !colorless_logo.is_empty() {
        for line in colorless_logo {
            logo.push(colorize(line.to_string()));
        }
    }

    // text of component
    //
    // component.icon component.name:
    // component.content
    let mut components_text: Vec<String> = Vec::new();

    if !cfg.components.is_empty() {
        for component in cfg.components {
            if cfg.oneline {
                components_text.push(colorize(format!(
                    "{}{}: {}",
                    component.icon.unwrap_or("".into()),
                    component.name,
                    component.content
                )));
            } else {
                components_text.push(colorize(format!(
                    "{}{}:",
                    component.icon.unwrap_or("".into()),
                    component.name
                )));
                components_text.push(colorize(format!("{}", component.content)));
            }
            if cfg.newline {
                components_text.push("".into());
            }
        }
    }

    // merge logo and component
    let mut output = logo;
    for (pos, item) in components_text.iter().enumerate() {
        if pos >= output.len() {
            output.push(item.into());
        } else {
            let spacing = " ".repeat(cfg.spacing);
            output[pos] = format!("{}{}{}", output[pos], spacing, item);
        }
    }

    output
}

/// Render and display text from config
///
/// Basically calls [render] under hood and prints every vector's item
pub fn display(cfg: Config) {
    let text = render(cfg);

    for line in text {
        println!("{}", line);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config;
    #[test]
    fn validate_rendered_text() {
        let rendered = render(Config {
            logo: config::Logo::Custom(vec![
                "S O M E    ".into(),
                "C U S T O M".into(),
                "L O G O    ".into(),
            ]),
            components: vec![
                config::Component {
                    name: "Component with an icon".into(),
                    icon: Some("* ".into()),
                    content: "Some component text".into(),
                },
                config::Component {
                    name: "Component without an icon".into(),
                    icon: None,
                    content: "Some component text".into(),
                },
                config::Component {
                    name: "Component with colored text".into(),
                    icon: None,
                    content: "[black]1[red]2[green]3[yellow]4[blue]5[magenta]6[cyan]7[white]8[_]9"
                        .into(),
                },
            ],
            newline: true,
            spacing: 1,
            oneline: false,
        });

        let expected = vec![
            "S O M E     * Component with an icon:",
            "C U S T O M Some component text",
            "L O G O     ",
            "Component without an icon:",
            "Some component text",
            "",
            "Component with colored text:",
            "\x1b[30m1\x1b[31m2\x1b[32m3\x1b[33m4\x1b[34m5\x1b[35m6\x1b[36m7\x1b[37m8\x1b[0m9",
            "",
        ];

        assert_eq!(rendered, expected);
    }
}
