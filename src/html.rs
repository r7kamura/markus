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
                FencedCodeBlock(info) => {
                    let language = info.split(' ').next().unwrap();
                    if language.is_empty() {
                        writer.push_str("<pre><code>");
                    } else {
                        writer.push_str(&format!(
                            r#"<pre><code class="language-{language}">"#,
                            language = language
                        ));
                    }
                }
                Heading(level) => {
                    writer.push_str(&format!("<h{}>", level as usize));
                }
                IndentedCodeBlock => {
                    writer.push_str("<pre><code>");
                }
                Paragraph => {
                    writer.push_str("<p>");
                }
            },
            End(tag) => match tag {
                FencedCodeBlock(_) | IndentedCodeBlock => {
                    writer.push_str("</code></pre>\n");
                }
                Heading(level) => {
                    writer.push_str(&format!("</h{}>\n", level as usize));
                }
                Paragraph => {
                    writer.push_str("</p>\n");
                }
            },
            Text(value) => {
                writer.push_str(value);
            }
            ThematicBreak => {
                writer.push_str("<hr />\n");
            }
        }
    }
}
