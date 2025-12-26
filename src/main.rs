extern crate anyhow;
extern crate handlebars;
extern crate mdbook;
extern crate regex;
extern crate serde;
extern crate serde_json;
extern crate uuid;

use anyhow::Result;
use mdbook::book::Book;
use mdbook::preprocess::{CmdPreprocessor, Preprocessor, PreprocessorContext};

use regex::Regex;

pub mod process_diagram;
use process_diagram::process_diagram;
pub mod handlebar_helpers;

fn main() {
    mdbook_preprocessor_boilerplate::run(
        MermaidAnimatePreprocessor,
        "An mdbook preprocessor for animating Mermaid diagrams", // CLI description
    );
}

struct MermaidAnimatePreprocessor;

fn run_section(section: &mdbook::book::BookItem) -> Result<mdbook::book::BookItem> {
    // dbg!("section: {}", &section);
    let section = match section {
        mdbook::book::BookItem::Chapter(chapter) => run_chapter(chapter)?,
        _ => unimplemented!(),
    };
    Ok(mdbook::BookItem::Chapter(section))
}

fn run_chapter(chapter: &mdbook::book::Chapter) -> Result<mdbook::book::Chapter> {
    let re = Regex::new(r#"(?ms)(?<mermaid><pre .*?class=\"mermaid\".*?</pre>)"#).unwrap();
    let mut count = 1;

    let mut replacements: Vec<(String, String)> = vec![];
    let mut data = chapter.content.clone();
    loop {
        let caps = re.captures(&data);
        if caps.is_none() {
            break;
        }
        let caps = caps.unwrap();
        // dbg!("caps: {:?}", &caps);
        let diagram = &caps["mermaid"];
        let processed_diagram = process_diagram(&diagram)?;
        // dbg!("before diagram: {}", &diagram);
        // dbg!("processed diagram: {}", &processed_diagram);
        let id = uuid::Uuid::new_v4();
        // dbg!("generated id: {}", &id);
        replacements.push((id.to_string(), processed_diagram.clone()));
        let mut dst = String::new();
        dst.push_str(&data[..caps.get(0).unwrap().start()]);
        dst.push_str(id.to_string().as_str());
        dst.push_str(&data[caps.get(0).unwrap().end()..]);
        data = dst;
    }

    let mut chapter = chapter.clone();
    for (id, processed_diagram) in replacements {
        data = data.replace(&id, &processed_diagram);
    }

    chapter.content = data;

    let sub_items: Vec<_> = chapter
        .sub_items
        .iter()
        .map(|c| run_section(c))
        .collect::<Result<Vec<_>>>()?;

    chapter.sub_items = sub_items;

    Ok(chapter.clone())
}

fn run_all(book: &mdbook::book::Book) -> Result<mdbook::book::Book> {
    let sections: &Vec<mdbook::book::BookItem> = &book
        .sections
        .iter()
        .map(|section| run_section(section))
        .collect::<Result<Vec<_>>>()?;

    let mut book: &mut Book = &mut book.clone();
    book.sections = sections.clone();
    Ok(book.clone())
}

impl Preprocessor for MermaidAnimatePreprocessor {
    fn name(&self) -> &str {
        "mermaid-animate"
    }

    fn run(&self, ctx: &PreprocessorContext, book: Book) -> Result<Book> {
        log::info!("Running mermaid-animate preprocessor");
        if let Some(nop_cfg) = ctx.config.get_preprocessor(self.name()) {
            if nop_cfg.contains_key("blow-up") {
                anyhow::bail!("Boom!!1!");
            }
        }

        let processed_book = run_all(&book);
        match processed_book {
            Err(e) => {
                eprintln!("Error during processing: {}", e);
                Ok(book)
            }
            Ok(b) => Ok(b),
        }
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer != "not-supported"
    }
}
