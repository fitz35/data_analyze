use std::sync::Mutex;
use std::ops::Range;

use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::data::linspace::Linspace;


const SERIE_DIVISION : usize = 500;

/// The goal of the function is to compress a dataserie to accelerate the plotting
/// it will cut the graph into piece en make the average of each piece
/// NOTE : The function take the borow of the data to avoid the memory allocation
#[cfg(not(feature = "parrallelize"))]
pub(crate) fn compress_data_serie(to_compress : Vec<(f32, f32)>, range_x : &Range<f32>, range_y : &Range<f32>) -> Vec<(f32, f32)> {
    // cut the range into SERIE_DIVISION pieces
    let x_linspace = Linspace::new(range_x.start as f64, range_x.end as f64, SERIE_DIVISION);
    let y_linspace = Linspace::new(range_y.start as f64, range_y.end as f64, SERIE_DIVISION);

    // create the hashmap to store the data
    // the key is the index of the piece in the x and y linspace
    // the value is a tuple with the sum of the x and y and the number of points
    // use welfort's algorithm
    let mut result_map : HashMap<(usize, usize), (f32, f32, usize)> = HashMap::new();
    for (x, y) in to_compress.into_iter() {
        let x_index = x_linspace.index_of(&(x as f64)).unwrap();
        let y_index = y_linspace.index_of(&(y as f64)).unwrap();
        let entry = result_map.entry((x_index, y_index)).or_insert((0.0, 0.0, 0));
        entry.2 += 1;
        entry.0 += (x - entry.0) / entry.2 as f32;
        entry.1 += (y - entry.1) / entry.2 as f32;
        
    }

    // create the result vector
    let mut result : Vec<(f32, f32)> = Vec::new();
    for (x, y, _) in result_map.values() {
        result.push((*x, *y));
    }
    
    result
}



/// The goal of the function is to compress a dataserie to accelerate the plotting
/// it will cut the graph into piece en make the average of each piece
/// NOTE : The function take the borow of the data to avoid a double memory allocation
#[cfg(feature = "parrallelize")]
pub(crate) fn compress_data_serie(to_compress : Vec<(f32, f32)>, range_x : &Range<f32>, range_y : &Range<f32>) -> Vec<(f32, f32)> {
    // cut the range into SERIE_DIVISION pieces

   

    
    let x_linspace = Linspace::new(range_x.start as f64, range_x.end as f64, SERIE_DIVISION);
    let y_linspace = Linspace::new(range_y.start as f64, range_y.end as f64, SERIE_DIVISION);

    let mut discrete_points : Vec<Mutex<(f64, f64, u64)>> = Vec::with_capacity(SERIE_DIVISION*SERIE_DIVISION);
    for _ in 0..SERIE_DIVISION*SERIE_DIVISION {
        discrete_points.push(Mutex::new((0.0, 0.0, 0)));
    }

    // use the welfort's algorithm to compress the data
    to_compress.into_par_iter().map(|(x, y)| {
        let x_index = x_linspace.index_of(&(x as f64)).unwrap();
        let y_index = y_linspace.index_of(&(y as f64)).unwrap();
        let index = x_index * SERIE_DIVISION + y_index;
        let mut value = discrete_points[index].lock().unwrap(); // block until the lock is available (in scope)
        value.2 += 1;
        value.0 += (x as f64 - value.0) / value.2 as f64;
        value.1 += (y as f64 - value.1) / value.2 as f64;
    }).collect::<Vec<_>>(); // need to collect to force the calcul of the iterator
    
    discrete_points.into_par_iter().filter_map(|mutex| {
        let value = mutex.lock().unwrap();
        if value.2 == 0 {
            return None;
        }
        Some((value.0 as f32, value.1 as f32))
    }).collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_dataset() {
        let to_compress: Vec<(f32, f32)> = Vec::new();
        let range_x = Range { start: 0.0, end: 10.0 };
        let range_y = Range { start: 0.0, end: 10.0 };

        let compressed = compress_data_serie(to_compress, &range_x, &range_y);

        assert_eq!(compressed.len(), 0);
    }

    #[test]
    fn test_single_point_dataset() {
        let to_compress = vec![(5.0, 5.0)];
        let range_x = Range { start: 0.0, end: 10.0 };
        let range_y = Range { start: 0.0, end: 10.0 };

        let compressed = compress_data_serie(to_compress, &range_x, &range_y);

        assert_eq!(compressed.len(), 1);
        assert_eq!(compressed[0], (5.0, 5.0));
    }

    #[test]
    fn test_multiple_points_single_division() {
        let to_compress = vec![(1.0, 1.0), (2.0, 2.0)];
        let range_x = Range { start: 0.0, end: 10.0 };
        let range_y = Range { start: 0.0, end: 10.0 };

        let compressed = compress_data_serie(to_compress, &range_x, &range_y);

        assert!(compressed.len() == 2);
    }

    #[test]
    fn test_points_spanning_multiple_divisions() {
        let to_compress = vec![(1.0, 1.0), (9.0, 9.0)];
        let range_x = Range { start: 0.0, end: 10.0 };
        let range_y = Range { start: 0.0, end: 10.0 };

        let compressed = compress_data_serie(to_compress, &range_x, &range_y);

        assert!(compressed.len() == 2);
    }

   
    #[test]
    fn test_large_dataset() {
        let to_compress = (0..(2 * SERIE_DIVISION * SERIE_DIVISION)).map(|x| (x as f32 % 10.0, x as f32 % 10.0)).collect();
        let range_x = Range { start: 0.0, end: 10.0 };
        let range_y = Range { start: 0.0, end: 10.0 };

        let compressed = compress_data_serie(to_compress, &range_x, &range_y);

        assert!(compressed.len() <= SERIE_DIVISION * SERIE_DIVISION);
        // Further checks can assess distribution and averaging correctness
    }
}