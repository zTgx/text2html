# [text2html](https://github.com/zTgx/text2html) [![Build Status](https://travis-ci.org/zTgx/text2html.svg?branch=master)](https://travis-ci.org/zTgx/text2html) [![crate](https://img.shields.io/crates/v/text2html.svg)](https://crates.io/crates/text2html) 

#### an exprimental project.
WIP  
A library render text to html in Rust.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
text2html = "0.0.1"
```

---
Example:
```rust
pub struct Content <'b> {
    pub c: &'b String,
}
impl <'b> Content <'b> {
    pub fn new(s: &'b String) -> Self {
        Content {
            c: s,
        }
    }
}
impl <'b> Text for Content <'b> {
    fn data_source(&mut self) -> String {
        self.c.to_owned()
    }
}

pub struct TextBuilder <'a> {
    pub sc: &'a String,
}
impl <'a> TextBuilder <'a> {
    pub fn new(s: &'a String) -> Self {
        TextBuilder {
            sc: s,
        }
    }
}
impl <'a> TextBuilder <'a> {
    pub fn build(&mut self) -> Box<dyn Text + 'a> {
        Box::new( Content::new(&self.sc) )
    }
}

fn main() {
    let text = "Hello world.".to_string();
    let mut builder = TextBuilder::new( &text ).build();
    HtmlRender::new().render(&mut builder);
}
```
