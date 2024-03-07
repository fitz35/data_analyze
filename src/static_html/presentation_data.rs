

use std::fs;
use std::path::Path;

use serde::Serialize as SerdeSerialize;
use serde_derive::{Deserialize, Serialize};

use crate::params::HTML_TEMPLATE;

use super::to_html::{ToHtmlDepth, ToTableOfContent};

// ------------------------------------- Ir -------------------------------------

/// This is the intermediate representation of the presentation data
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Hash, Eq, PartialOrd, Ord)]
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
            elements : ListElement::default()
        }
    }
}

impl Ir {
    /// convert the intermediate representation to html
    pub fn to_html(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut html_template = HTML_TEMPLATE.to_string();
        let html_content = self.elements.to_html(1);
        let table_of_content = self.elements.get_table_of_content(1);
        html_template = html_template.replace("<!--table of contents-->", &table_of_content);
        html_template = html_template.replace("<!--contents-->", &html_content);

        Ok(html_template)
    }

    pub fn add_elements(&mut self, element : ListElement) {
        self.elements.elements.extend(element.elements);
    }

    pub fn add_element(&mut self, element : Element) {
        self.elements.elements.push(element);
    
    }

    /// Wrapper around the new_from_dir function of ListElement
    pub fn new_from_file_system(path : &str) -> Result<Ir, Box<dyn std::error::Error>> {
        Ok(ListElement::new_from_dir(path)?.into())
    }

    pub fn get_elements(&self) -> &ListElement {
        &self.elements
    }
}

// ------------------------------------- ListElement -------------------------------------


/// represent a list of elements
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Hash, Eq, PartialOrd, Ord)]
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

impl Default for ListElement {
    fn default() -> Self {
        ListElement {
            elements : Vec::new()
        }
    }
}

impl ListElement {
    pub fn add_element(&mut self, element : Element) {
        self.elements.push(element);
    }

    pub fn add_elements(&mut self, elements : ListElement) {
        self.elements.extend(elements.elements);
    }

    /// Get a new list element from a directory
    /// WARN : return an error if the path is not a directory and if the directory does not contain only directories
    pub fn new_from_dir(path : &str) -> Result<ListElement, Box<dyn std::error::Error>> {
        let mut elements = Vec::new();
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if !path.is_dir() {
                return Err("the directory should only contain directories".into());
            }
            elements.push(Element::new_from_dir(path.to_str().unwrap())?);
        }
        Ok(elements.into())
    }

    pub fn get_elements(&self) -> &Vec<Element> {
        &self.elements
    }
}

// ------------------------------------- Element -------------------------------------

/// represent an element (title associated with content)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Hash, Eq, PartialOrd, Ord)]
pub struct Element {
    pub(crate) title : String,
    pub(crate) content : Vec<ContentElement>
}


impl Element {

    pub fn new(title : String, content : Vec<ContentElement>) -> Element {
        Element {
            title,
            content
        }
    }

    /// Get a new element from a directory
    pub fn new_from_dir(path : &str) -> Result<Element, Box<dyn std::error::Error>> {
        let path_o = Path::new(path);
        let title = path_o.file_name().unwrap().to_str().unwrap().to_string();
        let mut content = Vec::new();
        let mut entries = fs::read_dir(path_o)?.collect::<Result<Vec<_>, _>>()?;
        entries.sort_by(|a, b| a.file_name().cmp(&b.file_name()));
        for entry in entries {
            let path = entry.path();
            content.push(ContentElement::new_from_path(path.to_str().unwrap())?);
        }
        Ok(Element::new(title, content))
    }

    pub fn get_title(&self) -> &String {
        &self.title
    }

    pub fn get_content(&self) -> &Vec<ContentElement> {
        &self.content
    }
}

// ------------------------------------- ContentElement -------------------------------------

/// represent a content element (content or element)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Hash, Eq, PartialOrd, Ord)]
pub enum ContentElement {
    Content(Content),
    Element(Element),
}

impl ContentElement {
    pub fn new_from_path(path : &str) -> Result<ContentElement, Box<dyn std::error::Error>> {
        let path_o = Path::new(path);
        if path_o.is_dir() {
            Ok(Element::new_from_dir(path)?.into())
        } else {
            Ok(Content::new_from_path(path)?.into())
        }
    }
}

impl From<Content> for ContentElement {
    fn from(content : Content) -> ContentElement {
        ContentElement::Content(content)
    }
}

impl From<Element> for ContentElement {
    fn from(element : Element) -> ContentElement {
        ContentElement::Element(element)
    }
}

// ------------------------------------- Content -------------------------------------

const COLLAPSABLE_EXTENSION : &str = "collapsable";
const TEXT_EXTENSION : &str = "text";

/// handle the content of an elkeemnt (text or link
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Hash, Eq, PartialOrd, Ord)]
pub enum Content {
    Text(Text),
    Image(String),
    Array(Array),
    Collapsable(Collapsable<Content>)
}

impl Content {
    /// Create a new image with the path on the file system
    pub fn new_image(path : &str) -> Content {
        Content::Image(path.to_string())
    }

    /// Create a new content from a path
    pub fn new_from_path(path : &str) -> Result<Content, Box<dyn std::error::Error>> {
        let path_o = Path::new(path);
        if path_o.is_file() {
            let extension = path_o.extension();
            if let Some(extension) = extension {
                let extension = extension.to_str().unwrap();
                match extension {
                    "csv" =>                   Ok(Array::from_csv(path).into()),
                    "png" | "jpg" | "jpeg" =>  Ok(Content::new_image(path)),
                    COLLAPSABLE_EXTENSION =>   Ok(Collapsable::load_from_file(path)?.into()),
                    TEXT_EXTENSION =>          Ok(Text::load_from_file(path)?.into()),
                    _ =>                       Err(format!("the extension {} is not supported for the file {}", extension, path).into())
                }
            } else {
                let content : Content = serde_json::from_str(&fs::read_to_string(path)?)?;
                Ok(content)
            }
        } else {
            Err("the path should be a file".into())
        }
    }
}

impl From<Text> for Content {
    fn from(text : Text) -> Content {
        Content::Text(text)
    }
}

impl From<Array> for Content {
    fn from(array : Array) -> Content {
        Content::Array(array)
    }
}

impl From<Collapsable<Content>> for Content {
    fn from(collapsable : Collapsable<Content>) -> Content {
        Content::Collapsable(collapsable)
    }
}

// ------------------------------------- Content -------------------------------------

// ************************ Collapsable

/// represent a collapsable element
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Hash, Eq, PartialOrd, Ord)]
pub struct Collapsable<InnerContent> 
where InnerContent : SerdeSerialize
{
    pub(crate) summary : String,
    /// NOTE : the content of the collapsable element is not a list of elements but a list of content elements, avoid the including in the table of content
    pub(crate) content : Vec<InnerContent>,
}

impl<T> Collapsable<T> 
where T : SerdeSerialize
{
    pub fn new(summary : String, content : Vec<T>) -> Collapsable<T> {
        Collapsable {
            summary,
            content
        }
    }

    pub fn get_summary(&self) -> &String {
        &self.summary
    }

    pub fn get_content(&self) -> &Vec<T> {
        &self.content
    }
}

impl Collapsable<Content>{
    pub fn save_to_file(&self, dir_path : &str, file_name : &str) -> Result<(), Box<dyn std::error::Error>> {
        let path = Path::new(dir_path).join(format!("{}.{}", file_name, COLLAPSABLE_EXTENSION));
        fs::write(path, serde_json::to_string(self)?)?;
        Ok(())
    }

    pub fn load_from_file(path : &str) -> Result<Collapsable<Content>, Box<dyn std::error::Error>> {
        let path = Path::new(path);
        if !path.is_file() {
            return Err(format!("the path should be a file, path : {}", path.to_str().unwrap()).into());
        }
        if path.extension().unwrap().to_str().unwrap() != COLLAPSABLE_EXTENSION {
            return Err(format!("the file should have the correct extension, path : {}", path.to_str().unwrap()).into());
        }
        let content : Collapsable<Content> = serde_json::from_str(&fs::read_to_string(path)?)?;
        Ok(content)
    }
}

// ************************ Text

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Hash, Eq, PartialOrd, Ord)]
pub struct Text(Vec<TextContent>);

impl From<Vec<TextContent>> for Text {
    fn from(content : Vec<TextContent>) -> Text {
        Text(content)
    }
}

impl Text {
    pub fn save_to_file(&self, dir_path : &str, file_name : &str) -> Result<(), Box<dyn std::error::Error>> {
        let path = Path::new(dir_path).join(format!("{}.{}", file_name, TEXT_EXTENSION));
        fs::write(path, serde_json::to_string(self)?)?;
        Ok(())
    }

    pub fn load_from_file(path : &str) -> Result<Text, Box<dyn std::error::Error>> {
        let path = Path::new(path);
        if !path.is_file() {
            return Err(format!("the path should be a file, path : {}", path.to_str().unwrap()).into());
        }
        if path.extension().unwrap().to_str().unwrap() != TEXT_EXTENSION {
            return Err(format!("the file should have the correct extension, path : {}", path.to_str().unwrap()).into());
        }
        let content : Text = serde_json::from_str(&fs::read_to_string(path)?)?;
        Ok(content)
    }

    pub fn get_content(&self) -> &Vec<TextContent> {
        &self.0
    }
}


/// represent a text element
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Hash, Eq, PartialOrd, Ord)]
pub enum TextContent {
    Raw(String),
    Link(TextLink),
    Collapsable(Collapsable<TextContent>)
}

impl From<&str> for TextContent {
    fn from(s : &str) -> TextContent {
        TextContent::Raw(s.to_string())
    }
}

impl From<String> for TextContent {
    fn from(s : String) -> TextContent {
        TextContent::Raw(s)
    }
}

impl From<TextLink> for TextContent {
    fn from(l : TextLink) -> TextContent {
        TextContent::Link(l)
    }
}

impl From<Collapsable<TextContent>> for TextContent {
    fn from(c : Collapsable<TextContent>) -> TextContent {
        TextContent::Collapsable(c)
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Hash, Eq, PartialOrd, Ord)]
pub struct TextLink {
    pub(crate) href : String,
    pub(crate) text : String,
}

impl TextLink {
    pub fn new(href : String, text : String) -> TextLink {
        TextLink {
            href,
            text
        }
    }

    pub fn get_href(&self) -> &String {
        &self.href
    }

    pub fn get_text(&self) -> &String {
        &self.text
    }
}

// ************************ ArrayElement

/// represent an array of elements
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Hash, Eq, PartialOrd, Ord)]
pub struct Array{
    pub(crate) header : Vec<String>,
    pub(crate) data : Vec<Vec<String>>,
}

impl Array {
    pub fn new(header : Vec<String>, data : Vec<Vec<String>>) -> Array {
        Array {
            header,
            data
        }
    }

    pub fn from_csv(path : &str) -> Array {
        let mut csv_reader = csv::ReaderBuilder::new().has_headers(false).from_path(path).unwrap();
        let mut header = Vec::new();
        let mut data = Vec::new();
        for result in csv_reader.records() {
            let record = result.unwrap();
            if header.is_empty() {
                header = record.iter().map(|s| s.to_string()).collect();
            } else {
                data.push(record.iter().map(|s| s.to_string()).collect());
            }
        }
        Array {
            header,
            data
        }
    }

    pub fn get_header(&self) -> &Vec<String> {
        &self.header
    }

    pub fn get_data(&self) -> &Vec<Vec<String>> {
        &self.data
    }
}

