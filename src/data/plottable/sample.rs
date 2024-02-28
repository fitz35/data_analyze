use super::key::SerieKey;







/// Define a sample linked to a key
pub trait Sample<Key>
    where 
        Self : Sized + Send + Sync,
        Key : SerieKey
{
    /// Load a sample from a file path
    fn new_from_file_path(file_path : &str) -> Result<Self, Box<dyn std::error::Error>>;

    /// Get the value of data (as f32), associated to the given key
    fn get_numeric_value(&self, key : &Key) -> f32;

    /// Get the value of data (as string), associated to the given key
    fn get_string_value(&self, key : &Key) -> String;

}