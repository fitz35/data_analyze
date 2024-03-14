#[cfg(feature = "parrallelize")]
use rayon::iter::{IntoParallelRefIterator, ParallelIterator, IndexedParallelIterator};

use crate::data::plottable::key::SerieKey;



/// define a mask, true to keep the data
pub struct Mask {
    mask : Vec<bool>,
}

impl From<Vec<bool>> for Mask {
    fn from(mask : Vec<bool>) -> Self {
        Self {
            mask,
        }
    }
}

impl Mask {
    pub fn new(size : usize) -> Self {
        Self {
            mask : vec![true; size],
        }
    }

    pub fn merge(&mut self, mask : &Vec<bool>) {
        for (i, m) in mask.iter().enumerate() {
            self.mask[i] = self.mask[i] && *m;
        }
    }

    #[cfg(not(feature = "parrallelize"))]
    pub fn apply<U>(&self, data : &Vec<U>) -> Vec<U> where U : Clone {
        let mut filtered_data = Vec::new();
        for (i, d) in data.iter().enumerate() {
            if self.mask[i] {
                filtered_data.push(d.clone());
            }
        }
        filtered_data
    }

    #[cfg(feature = "parrallelize")]
    pub fn apply<U>(&self, data : &Vec<U>) -> Vec<U> where U : Clone + Send + Sync {

        data.par_iter().enumerate().filter(|(i, _)| self.mask[*i]).map(|(_, d)| d.clone()).collect()
    }
}




/// Define a filter
pub struct Filter<Key> 
where 
    Key : SerieKey
{
    key : Key,
    filter_number : Option<Box<dyn Fn(f32) -> bool>>,
    filter_str : Option<Box<dyn Fn(&str) -> bool>>,
}

impl<Key> Filter<Key>
where 
    Key : SerieKey
{
    pub fn new_number<F>(key : Key, filter_fn : F) -> Self
    where 
        F : Fn(f32) -> bool + 'static
    {
        assert!(key.is_numeric());
        Self {
            key,
            filter_number : Some(Box::new(filter_fn)),
            filter_str : None,
        }
    }

    pub fn new_str<F>(key : Key, filter_fn : F) -> Self
    where 
        F : Fn(&str) -> bool + 'static
    {
        assert!(key.is_string());
        Self {
            key,
            filter_number : None,
            filter_str : Some(Box::new(filter_fn)),
        }
    }

    pub fn get_key(&self) -> &Key {
        &self.key
    }

    pub fn get_filter_number(&self) -> &Box<dyn Fn(f32) -> bool> {
        self.filter_number.as_ref().unwrap()
    }

    pub fn get_filter_str(&self) -> &Box<dyn Fn(&str) -> bool> {
        self.filter_str.as_ref().unwrap()
    }
}


