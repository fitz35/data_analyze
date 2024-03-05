

use std::fs;
use std::path::Path;

use serde_derive::{Deserialize, Serialize};

use crate::params::RESSOURCES_DIR;

use super::to_html::ToHtmlDepth;



/// This is the intermediate representation of the presentation data
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Hash)]
pub struct Ir {
    pub(crate) elements : ListElement
}

impl From<ListElement> for Ir {
    fn from(elements : ListElement) -> Ir {
        Ir {
            elements
        }
    }
}

impl Default for Ir {
    fn default() -> Self {
        Ir {
            elements : ListElement::new(Vec::new())
        }
    }
}

impl Ir {
    /// convert the intermediate representation to html
    pub fn to_html(&self) -> Result<String, Box<dyn std::error::Error>> {
        let html_template_path = Path::new(RESSOURCES_DIR).join("static.html");
        let mut html_template = fs::read_to_string(html_template_path)?;
        let html_content = self.elements.to_html(1);
        let table_of_content = self.elements.get_table_of_content(1).unwrap();
        html_template = html_template.replace("<!--table of contents-->", &table_of_content);
        html_template = html_template.replace("<!--contents-->", &html_content);

        Ok(html_template)
    }

    pub fn add_elements(&mut self, element : ListElement) {
        self.elements.elements.extend(element.elements);
    }
}

/// represent a list of elements
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Hash)]
pub struct ListElement {
    pub(crate) elements : Vec<Element>,
}

impl From<Vec<Element>> for ListElement {
    fn from(elements : Vec<Element>) -> ListElement {
        ListElement {
            elements
        }
    }
}

impl ListElement {
    pub fn new(elements : Vec<Element>) -> ListElement {
        ListElement {
            elements
        }
    }
}

/// represent an element (title associated with content)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Hash)]
pub struct Element {
    pub(crate) title : String,
    pub(crate) content : ContentElement
}

impl Element {
    pub fn new(title : String, content : ContentElement) -> Element {
        Element {
            title,
            content
        }
    }
}


/// handle the content of an elkeemnt (text or link
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Hash)]
pub enum ContentElement {
    Text(Vec<TextElement>),
    Image(String),
    Array(ArrayElement),
    Collapsable(CollapsableElement),
    Elements(ListElement),
}



/// represent a collapsable element
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Hash)]
pub struct CollapsableElement {
    pub(crate) summary : String,
    /// NOTE : the content of the collapsable element is not a list of elements but a list of content elements, avoid the including in the table of content
    pub(crate) content : Vec<ContentElement>,
}

impl CollapsableElement {
    pub fn new(summary : String, content : Vec<ContentElement>) -> CollapsableElement {
        CollapsableElement {
            summary,
            content
        }
    }
}


/// represent a text element
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Hash)]
pub enum TextElement {
    Raw(String),
    Link(TextLinkElement),
}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Hash)]
pub struct TextLinkElement {
    pub(crate) href : String,
    pub(crate) text : String,
}

impl TextLinkElement {
    pub fn new(href : String, text : String) -> TextLinkElement {
        TextLinkElement {
            href,
            text
        }
    }
}

/// represent an array of elements
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Hash)]
pub struct ArrayElement{
    pub(crate) header : Vec<String>,
    pub(crate) data : Vec<Vec<String>>,
}

impl ArrayElement {
    pub fn new(header : Vec<String>, data : Vec<Vec<String>>) -> ArrayElement {
        ArrayElement {
            header,
            data
        }
    }
}

