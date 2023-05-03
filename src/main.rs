use capitalize::Capitalize;
use clap::Parser;
use indoc::formatdoc;
use std::env::current_dir;
use std::fs::{create_dir_all, File};
use std::io::{prelude::*, BufWriter};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Command
    #[arg(short, long)]
    command: String,

    /// File name
    #[arg(short, long)]
    name: String,

    /// Fields
    #[arg(short, long)]
    fields: String,

    /// Description
    #[arg(short, long)]
    description: String,
}

fn main() {
    let binding = current_dir().expect("Can't get current directory");
    let path: &str = match binding.as_os_str().to_str() {
        Some(s) => s,
        _ => panic!("Can't get current directory"),
    };
    let args = Args::parse();

    if args.command == String::from("g") {
        println!("generating...");

        let name = Name::from(&args.name);

        let data = formatdoc! {"
            <?php
            /**
             * {name} Template.
             */

            // Support custom \"anchor\" values.
            $anchor = '';
            if ( ! empty( $block['anchor'] ) ) {{
                $anchor = 'id=\"' . esc_attr( $block['anchor'] ) . '\" ';
            }}

            // Create class attribute allowing for custom \"className\" and \"align\" values.
            $class_name = '{file_name}';
            if ( ! empty( $block['className'] ) ) {{
                $class_name .= ' ' . $block['className'];
            }}
            if ( ! empty( $block['align'] ) ) {{
                $class_name .= ' align' . $block['align'];
            }}

            // Load values and assign defaults.
           
            {fields}

            ?>

            <section <?= $anchor; ?>class=\"<?= esc_attr( $class_name ); ?> py-section\">
                <div class=\"grid-container\">
                </div>
            </section>
            ",
            name = name.human_readable,
            file_name = name.file_readable,
            fields = parse_fields(args.fields)
        };

        create_dir_all(format!("{}/blocks/{}", path, name.file_readable))
            .expect("Can't create directory");

        let php_template_file = File::create(format!(
            "{}/blocks/{}/{}.php",
            path, name.file_readable, name.file_readable
        ))
        .expect("Can't create file");

        let mut php_template_file = BufWriter::new(php_template_file);
        php_template_file
            .write_all(data.as_bytes())
            .expect("Can't write to file");

        let data = formatdoc!(
            "
            {{
              \"name\": \"acf/{file_name}\",
              \"title\": \"{name}\",
              \"description\": \"{description}\",
              \"style\": [ \"file:../../style.css\" ],
              \"category\": \"formatting\",
              \"icon\": \"admin-comments\",
              \"keywords\": [\"custom\", \"block\"],
              \"acf\": {{
                  \"mode\": \"preview\",
                  \"renderTemplate\": \"{file_name}.php\"
              }},
              \"align\": \"full\"
            }}
            ",
            name = name.human_readable,
            file_name = name.file_readable,
            description = args.description,
        );

        let block_json = File::create(format!("{}/blocks/{}/block.json", path, name.file_readable))
            .expect("Can't create file");

        let mut block_json = BufWriter::new(block_json);
        block_json
            .write_all(data.as_bytes())
            .expect("Can't write to file");

        println!("{} files generated!", name.human_readable);
    };
}

fn humanize_name(name: &String) -> String {
    name.split("-")
        .map(|s| s.capitalize())
        .collect::<Vec<String>>()
        .join(" ")
}

fn parse_fields(fields: String) -> String {
    let fields: Vec<&str> = fields.split_whitespace().collect();

    let appended: Vec<String> = fields
        .into_iter()
        .map(|field| {
            formatdoc!(
                "
            ${field} = get_field( '{field}' ) ?: null;
            ",
                field = field,
            )
        })
        .collect();

    appended.join("")
}

struct Name {
    human_readable: String,
    file_readable: String,
}

impl Name {
    fn from(s: &str) -> Name {
        let is_file_name = s.contains("-");

        if is_file_name {
            Name {
                human_readable: humanize_name(&s.to_string()),
                file_readable: s.to_string(),
            }
        } else {
            Name {
                human_readable: s.to_string(),
                file_readable: s.to_lowercase().replace(" ", "-"),
            }
        }
    }
}

mod tests {
    #[cfg(test)]
    use crate::humanize_name;

    #[test]
    fn humanizes() {
        struct Test {
            input: String,
            output: String,
        }

        let tests = vec![
            Test {
                input: "hello".to_string(),
                output: "Hello".to_string(),
            },
            Test {
                input: "hey-jude".to_string(),
                output: "Hey Jude".to_string(),
            },
        ];

        tests
            .iter()
            .for_each(|t| assert_eq!(humanize_name(&t.input), t.output));
    }

    #[cfg(test)]
    use crate::Name;

    #[test]
    fn parse_name_human_readable() {
        struct Test {
            input: String,
            output: String,
        }

        let tests = vec![
            Test {
                input: "My Awesome Name".to_string(),
                output: "My Awesome Name".to_string(),
            },
            Test {
                input: "my-awesome-name".to_string(),
                output: "My Awesome Name".to_string(),
            },
            Test {
                input: "Name".to_string(),
                output: "Name".to_string(),
            },
            Test {
                input: "awesome".to_string(),
                output: "awesome".to_string(),
            },
        ];

        tests
            .iter()
            .for_each(|t| assert_eq!(Name::from(&t.input).human_readable, t.output));
    }

    #[test]
    fn parse_name_file_readable() {
        struct Test {
            input: String,
            output: String,
        }

        let tests = vec![
            Test {
                input: "My Awesome Name".to_string(),
                output: "my-awesome-name".to_string(),
            },
            Test {
                input: "my-awesome-name".to_string(),
                output: "my-awesome-name".to_string(),
            },
            Test {
                input: "Who".to_string(),
                output: "who".to_string(),
            },
            Test {
                input: "my".to_string(),
                output: "my".to_string(),
            },
        ];

        tests
            .iter()
            .for_each(|t| assert_eq!(Name::from(&t.input).file_readable, t.output));
    }

    #[test]
    fn parse_numbers_to_human_readable() {
        struct Test {
            input: String,
            output: String,
        }

        let tests = vec![
            Test {
                input: "my-cool-file-v23".to_string(),
                output: "My Cool File V23".to_string(),
            },
            Test {
                input: "hey-83".to_string(),
                output: "Hey 83".to_string(),
            },
            Test {
                input: "83-hey".to_string(),
                output: "83 Hey".to_string(),
            },
        ];

        tests.iter().for_each(|t| {
            assert_eq!(Name::from(&t.input).human_readable, t.output);
        });
    }

    #[test]
    fn parse_numbers_to_file_readable() {
        struct Test {
            input: String,
            output: String,
        }

        let tests = vec![
            Test {
                input: "My Cool File V23".to_string(),
                output: "my-cool-file-v23".to_string(),
            },
            Test {
                input: "Hey 83".to_string(),
                output: "hey-83".to_string(),
            },
            Test {
                input: "83 Hey".to_string(),
                output: "83-hey".to_string(),
            },
        ];

        tests.iter().for_each(|t| {
            assert_eq!(Name::from(&t.input).file_readable, t.output);
        });
    }
}
