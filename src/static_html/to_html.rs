use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use super::presentation_data::{ArrayElement, CollapsableElement, ContentElement, Element, ListElement, TextElement, TextLinkElement};


fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}


/// This trait is used to convert the data to html and to get the table of content
pub trait ToHtmlDepth {
    fn to_html(&self, depth : usize) -> String;
    fn get_table_of_content(&self, depth : usize) -> Option<String>;
}

impl ToHtmlDepth for String {
    fn to_html(&self, _depth : usize) -> String {
        self.clone()
    }

    fn get_table_of_content(&self, _depth : usize) -> Option<String> {
        None
    }
}

impl ToHtmlDepth for ListElement {
    fn to_html(&self, depth : usize) -> String {
        let result = self.elements.iter().map(|e| e.to_html(depth)).collect::<Vec<String>>().join("\n");
        result
    }

    fn get_table_of_content(&self, depth : usize) -> Option<String> {
        let mut result = String::new();
        result.push_str("<ul>");
        for e in self.elements.iter() {
            if let Some(e) = e.get_table_of_content(depth) {
                result.push_str(&e);
            }
        }
        result.push_str("</ul>");

        Some(result)
    }
}


impl ToHtmlDepth for Element {
    fn to_html(&self, depth : usize) -> String {
        let hash = calculate_hash(&self).to_string();
        let mut result = String::new();
        result.push_str(&format!("<h{} id=\"{}\">{}</h{}>", depth, hash, &self.title, depth));
        result.push_str(&self.content.to_html(depth + 1));
        result
    }

    fn get_table_of_content(&self, depth : usize) -> Option<String> {
        let hash = calculate_hash(&self).to_string();
        let href = format!("#{}", hash);
        let mut result = String::new();
        result.push_str(&format!("<li><a href=\"{}\">- {}</a></li>", href, &self.title));
        if let Some(e) = self.content.get_table_of_content(depth + 1) {
            result.push_str(&e);
        }
        Some(result)
    }
}


impl ToHtmlDepth for ContentElement {
    fn to_html(&self, depth : usize) -> String {
        match self {
            ContentElement::Text(t) => {
                let mut result = String::new();
                result.push_str("<div>");
                for e in t.iter() {
                    result.push_str(&e.to_html(depth));
                }
                result.push_str("</div>");
                result
            },
            ContentElement::Image(s) => {
                format!("<img src=\"{}\"/>", s).to_html(depth)
            },
            ContentElement::Array(a) => a.to_html(depth),
            ContentElement::Collapsable(e) => e.to_html(depth),
            ContentElement::Elements(e) => e.to_html(depth),
        }
    }

    fn get_table_of_content(&self, depth : usize) -> Option<String> {
        match self {
            ContentElement::Elements(e) => e.get_table_of_content(depth),
            _ => None
        }
    }
}

impl ToHtmlDepth for CollapsableElement {
    fn to_html(&self, depth : usize) -> String {
        let mut result = String::new();
        result.push_str(&format!("<details><summary>{}</summary>", &self.summary));
        for e in self.content.iter() {
            result.push_str(&e.to_html(depth));
        }
        result.push_str("</details>");
        result
    }

    fn get_table_of_content(&self, _depth : usize) -> Option<String> {
        None
    }
}

impl ToHtmlDepth for TextElement {
    fn to_html(&self, depth : usize) -> String {
        match self {
            TextElement::Raw(s) => s.to_html(depth),
            TextElement::Link(l) => l.to_html(depth),
        }
    }

    fn get_table_of_content(&self, _depth : usize) -> Option<String> {
        None
    }
}


impl ToHtmlDepth for TextLinkElement {
    fn to_html(&self, depth : usize) -> String {
        format!("<a href=\"{}\">{}</a>", &self.href, &self.text).to_html(depth)
    }

    fn get_table_of_content(&self, _depth : usize) -> Option<String> {
        None
    }
}


impl ToHtmlDepth for ArrayElement {
    fn to_html(&self, _depth : usize) -> String {
        let mut result = String::new();
        result.push_str("<table class=\"custom-table\">");
        result.push_str("<thead>");
        result.push_str("<tr>");
        for e in self.header.iter() {
            result.push_str(&format!("<th>{}</th>", e));
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

    fn get_table_of_content(&self, _depth : usize) -> Option<String> {
        None
    }
}