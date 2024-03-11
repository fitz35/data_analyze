use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use serde::Serialize;

use super::presentation_data::{Array, Collapsable, Content, ContentElement, Element, ListElement, Text, TextContent, TextLink};


fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn format_title(title : &str) -> String {
    title.replace("_", " ")
}


/// This trait is used to convert the data to html
pub trait ToHtmlDepth {
    fn to_html(&self, depth : usize) -> String;
}

impl ToHtmlDepth for String {
    fn to_html(&self, _depth : usize) -> String {
        self.clone()
    }
}

/// this trait is used to get the table of content
pub trait ToTableOfContent {
    fn get_table_of_content(&self, depth : usize) -> String;
}

impl ToHtmlDepth for ListElement {
    fn to_html(&self, depth : usize) -> String {
        let result = self.elements.iter().map(|e| e.to_html(depth)).collect::<Vec<String>>().join("\n");
        result
    }
}

impl ToTableOfContent for ListElement {
    fn get_table_of_content(&self, depth : usize) -> String {
        let mut result = String::new();
        if self.elements.len() == 0 {
            return result;
        }
        result.push_str("<ul>");
        for element in self.elements.iter() {
            let toc = element.get_table_of_content(depth);
            result.push_str(&toc);
        }
        result.push_str("</ul>");

        result
    }
    
}

impl ToHtmlDepth for Element {
    fn to_html(&self, depth : usize) -> String {
        let hash = calculate_hash(&self).to_string();
        let mut result = String::new();
        result.push_str(&format!("<h{} id=\"{}\">{}</h{}>", depth, hash, format_title(&self.title), depth));
        for e in self.content.iter() {
            result.push_str(&e.to_html(depth + 1));
        }
        result
    }
}

impl ToTableOfContent for Element {
    fn get_table_of_content(&self, depth : usize) -> String {
        let hash = calculate_hash(&self).to_string();
        let href = format!("#{}", hash);
        let mut result = String::new();
        result.push_str(&format!("<li><a href=\"{}\">- {}</a></li>", href, format_title(&self.title)));
        if self.content.len() != 0 {
            result.push_str("<ul>");
        }
        for e in self.content.iter() {
            let toc = e.get_table_of_content(depth + 1);
            result.push_str(&toc);
        }
        if self.content.len() != 0 {
            result.push_str("</ul>");
        }
        result
    }
}

impl ToHtmlDepth for ContentElement {
    fn to_html(&self, depth : usize) -> String {
        match self {
            ContentElement::Content(c) => c.to_html(depth),
            ContentElement::Element(e) => e.to_html(depth),
        }
    }
}

impl ToTableOfContent for ContentElement {
    fn get_table_of_content(&self, depth : usize) -> String {
        match self {
            ContentElement::Content(_) => String::new(),
            ContentElement::Element(e) => e.get_table_of_content(depth),
        }
    }
}

impl ToHtmlDepth for Content {
    fn to_html(&self, depth : usize) -> String {
        match self {
            Content::Text(t) => t.to_html(depth),
            Content::Image(s) => {
                format!("<img src=\"{}\"/>", s).to_html(depth)
            },
            Content::Array(a) => a.to_html(depth),
            Content::Collapsable(e) => e.to_html(depth),
        }
    }
}

impl<T> ToHtmlDepth for Collapsable<T> 
where T: ToHtmlDepth + Serialize {
    fn to_html(&self, depth : usize) -> String {
        let mut result = String::new();
        result.push_str(&format!("<details><summary>{}</summary>", &self.summary));
        for e in self.content.iter() {
            result.push_str(&e.to_html(depth));
        }
        result.push_str("</details>");
        result
    }
}

impl ToHtmlDepth for Text {
    fn to_html(&self, depth : usize) -> String {
        let mut result = String::new();
        for e in self.get_content().iter() {
            result.push_str(&e.to_html(depth));
        }
        result = result.replace("\n", "<br>");
        result
    }
}

impl ToHtmlDepth for TextContent {
    fn to_html(&self, depth : usize) -> String {
        match self {
            TextContent::Raw(s) => s.to_html(depth),
            TextContent::Link(l) => l.to_html(depth),
            TextContent::Collapsable(c) => c.to_html(depth),
        }
    }
}


impl ToHtmlDepth for TextLink {
    fn to_html(&self, depth : usize) -> String {
        format!("<a href=\"{}\">{}</a>", &self.href, &self.text).to_html(depth)
    }
}


impl ToHtmlDepth for Array {
    fn to_html(&self, _depth : usize) -> String {
        let mut result = String::new();
        result.push_str("<table class=\"custom-table\">");
        result.push_str("<thead>");
        result.push_str("<tr>");
        for e in self.header.iter() {
            result.push_str(&format!("<th>{}</th>", format_title(e)));
        }
        result.push_str("</tr>");
        result.push_str("</thead>");
        result.push_str("<tbody>");
        for row in self.data.iter() {
            result.push_str("<tr>");
            for e in row.iter() {
                result.push_str(&format!("<td>{}</td>", e));
            }
            result.push_str("</tr>");
        }
        result.push_str("</tbody>");
        result.push_str("</table>");
        result
    }
}