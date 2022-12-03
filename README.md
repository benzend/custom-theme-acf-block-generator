# Custom Theme ACF Block Generator
A blazingly fast acf block generator for custom wordpress themes written in Rust.

See [this post](https://www.advancedcustomfields.com/resources/blocks/) for details surrounding ACF Blocks

## Installation
Make sure you have Rustup installed on your machine. You can find the installation [here](https://www.rust-lang.org/tools/install).

Clone this repo
```
git clone https://github.com/benzend/custom-theme-acf-block-generator.git
```

Now with the repo and rustup installed, navigate to where you cloned this repo.

Run this command to build binaries (cargo comes from rustup)
```
cargo build --release
```

You can find the binaries in `custom-theme-acf-block-generator/build/release/custom-theme-acf-block-generator`

To run it, you can simply call the binary file
```
./build/release/custom-theme-acf-block-generator
```

To make this accessable globally add an _alias_
```
alias acfblockgen='<full-path-to-binary-file>'
```

## How to use
To generate a block
```
acfblockgen -command g -name your-block-name -description "Your block description" -fields "background title description"
```

You can also run this to see the list of arguments
```
acfblockgen --help
```

## What is it actually doing
When you run this in the root of your project
```
acfblockgen -command g -name spicy-block -description "This is a spicy block" -fields "background title description"
```

This will generate a new directory with most of the needed pieces to create a custom acf block.

Before generation
```
your-project
--- index.php
--- function.php
--- package.json
--- README.md
```

After
```
your-project
--- index.php
--- function.php
--- package.json
--- README.md
--- blocks
------- spicy-block
----------- block.json
----------- spicy-block.php
```

In `block.json`
```
{
  "name": "acf/spicy-block",
  "title": "spicy-block",
  "description": "This is a spicy block",
  "style": [ "file:../../style.css" ],
  "category": "formatting",
  "icon": "admin-comments",
  "keywords": ["custom", "block"],
  "acf": {
      "mode": "preview",
      "renderTemplate": "spicy-block.php"
  },
  "align": "full"
}
```

In `spicy-block.php`
```
<?php
/**
 * spicy-block Template.
 */

// Support custom "anchor" values.
$anchor = '';
if ( ! empty( $block['anchor'] ) ) {
    $anchor = 'id="' . esc_attr( $block['anchor'] ) . '" ';
}

// Create class attribute allowing for custom "className" and "align" values.
$class_name = 'spicy-block';
if ( ! empty( $block['className'] ) ) {
    $class_name .= ' ' . $block['className'];
}
if ( ! empty( $block['align'] ) ) {
    $class_name .= ' align' . $block['align'];
}

// Load values and assign defaults.

$background = get_field( 'background' ) ?: null;
$title = get_field( 'title' ) ?: null;
$description = get_field( 'description' ) ?: null;

?>
```

## Args
Block name
```
-name | -n
String - kebab or snake case
```

Block description
```
-description | -d
String - any
```

Block fields
```
-fields | -d
String - snake case
```
