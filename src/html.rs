use crate::types::Event;
use crate::types::Event::*;
use crate::types::Tag::*;
use std::iter::Iterator;

pub fn push_html<'a, I>(writer: &mut String, iterator: I)
where
    I: Iterator<Item = Event<'a>>,
{
    for event in iterator {
        match event {
            Begin(tag) => match tag {
                Paragraph => {
                    writer.push_str("<p>");
                }
                Heading(level) => {
                    writer.push_str(&format!("<h{}>", level as usize));
                }
            },
            End(tag) => match tag {
                Paragraph => {
                    writer.push_str("</p>");
                }
                Heading(level) => {
                    writer.push_str(&format!("</h{}>", level as usize));
                }
            },
            Text(value) => {
                writer.push_str(value);
            }
        }
    }
}
