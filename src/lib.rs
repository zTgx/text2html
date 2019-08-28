use std::marker::PhantomData;
use std::fs::{OpenOptions};
use uuid::Uuid;
use std::io::Write;

pub trait Render <'r> {
    fn render(&mut self, text: &mut Box<dyn Text + 'r>);
}

pub trait Text {
    fn data_source(&mut self) -> String;
}

pub struct HtmlRender <'h> {
    pub css: String,
    phantom: PhantomData<&'h String>,
}

impl <'h> HtmlRender <'h> {
    pub fn new() -> Self {
        HtmlRender {
            css: "".to_string(),
            phantom: PhantomData,
        }
    }
}

impl <'h> Render <'h> for HtmlRender <'h> {
    fn render(&mut self, text: &mut Box<dyn Text + 'h>) -> () {
        let content = text.data_source();
        let uuid = format!("{}", Uuid::new_v4());
        let name = uuid + ".html";
        let mut file = OpenOptions::new().write(true).create(true).open(name).unwrap();
        file.write_all(content.as_bytes()).unwrap();
    }
}
