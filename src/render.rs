//! Rendering config into text
use crate::config::{Config, Logo};

/// [config::Config] to [Vec<String>]
///
/// Takes passed [Config],
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
    let mut logo: Vec<String> = Vec::new();

    // write logo to variable
    match cfg.logo {
        Logo::Os => {
            unimplemented!();
        }
        Logo::Custom(provided_logo) => {
            logo = provided_logo;
        }
        Logo::Disabled => {}
    }

    // text of component
    //
    // component.icon component.name:
    // component.content
    let mut components_text: Vec<String> = Vec::new();

    if !cfg.components.is_empty() {
        for component in cfg.components {
            components_text.push(format!(
                "{}{}:",
                component.icon.unwrap_or("".into()),
                component.name,
            ));
            components_text.push(format!("{}", component.content,));
            components_text.push("".into()); // TODO: allow user configurate whether there's new line after component or not
        }
    }

    // merge logo and component
    let mut output = logo;
    for (pos, item) in components_text.iter().enumerate() {
        if pos >= output.len() {
            output.push(item.into());
        } else {
            output[pos] = format!("{} {}", output[pos], item); // TODO: allow user configurate amount of spacing between logo and text
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
            ],
        });

        let expected = vec![
            "S O M E     * Component with an icon:",
            "C U S T O M Some component text",
            "L O G O     ",
            "Component without an icon:",
            "Some component text",
            "",
        ];

        assert_eq!(rendered, expected);
    }
}
