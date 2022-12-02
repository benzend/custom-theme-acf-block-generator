use clap::Parser;
use std::fs::{File, create_dir_all};
use std::env::current_dir;
use std::io::{prelude::*, BufWriter};
use indoc::formatdoc;

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
        _ => panic!("Can't get current directory")
    };
   let args = Args::parse();

   if args.command == String::from("g") {
       println!("generating...");
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
            $class_name = '{name}';
            if ( ! empty( $block['className'] ) ) {{
                $class_name .= ' ' . $block['className'];
            }}
            if ( ! empty( $block['align'] ) ) {{
                $class_name .= ' align' . $block['align'];
            }}

            // Load values and assign defaults.
           
            {fields}

            ?>
            ",
            name = args.name,
            fields = parse_fields(args.fields)
        };

        create_dir_all(format!("{}/blocks/{}", path, args.name)).expect("Can't create directory");

        let php_template_file = File::create(
            format!("{}/blocks/{}/{}.php", path, args.name, args.name)
        ).expect("Can't create file");

        let mut php_template_file = BufWriter::new(php_template_file);
        php_template_file.write_all(data.as_bytes()).expect("Can't write to file");


        let data = formatdoc! ("
            {{
              \"name\": \"acf/{name}\",
              \"title\": \"{name}\",
              \"description\": \"{description}\",
              \"style\": [ \"file:../../style.css\" ],
              \"category\": \"formatting\",
              \"icon\": \"admin-comments\",
              \"keywords\": [\"custom\", \"block\"],
              \"acf\": {{
                  \"mode\": \"preview\",
                  \"renderTemplate\": \"{name}.php\"
              }},
              \"align\": \"full\"
            }}
            ",
            name = args.name,
            description = args.description,
        );

        let block_json = File::create(
            format!("{}/blocks/{}/block.json", path, args.name)
        ).expect("Can't create file");
        
        let mut block_json = BufWriter::new(block_json);
        block_json.write_all(data.as_bytes()).expect("Can't write to file");
   };
}

fn parse_fields(fields: String) -> String {
    let fields: Vec<&str> = fields.split_whitespace().collect();

    let appended: Vec<String> = fields.into_iter().map(|field| {
       formatdoc!("
            ${field} = get_field( '{field}' ) ?: null;
            ",
            field = field,
        )
    }).collect();

    appended.join("")
}
