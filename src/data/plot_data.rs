use std::collections::HashMap;
use std::ops::Range;

use crate::stats::stats_serie::{MetricName, StatsSerie};





/// a (x, y) point
type Point = (f32, f32);

/// represent pluggable data, indexed by a legend, for one graph
#[derive(Debug, Clone)]
pub struct PlotSeries {
    data:  HashMap<String, Vec<Point>>
}

impl PlotSeries {
    pub fn new() -> Self {
        Self {
            data : HashMap::new(),
        }
    }

    pub fn add(&mut self, legend : String, point : Point) {
        let entry = self.data.entry(legend).or_insert(Vec::new());
        entry.push(point);
    }

    /// aggregate the data and combine the value with the same x value with a specified metric
    pub fn apply_aggregator(&self, aggregator : MetricName) -> Result<PlotSeries, Box<dyn std::error::Error>> {
        let mut aggregated_data = HashMap::new();
        for (key, serie) in self.data.iter() {
            let mut serie = serie.clone();
            serie.sort_by(|(x1, _), (x2, _)| x1.partial_cmp(x2).unwrap());
            let mut aggregated_serie = Vec::new(); // new serie
            let mut current_x = f32::MIN; // current x value for the aggregation
            let mut current_y = Vec::new(); // all the y values for the current x value
            for (x, y) in serie.iter() {
                if *x == current_x { // if the x value is the same as the current one, add the y value to the current y values
                    current_y.push(*y);
                } else { // if the x value is different, calculate the metrics and reset the current x and y values
                    if current_y.len() != 0 {
                        // calculate the metrics
                        let stats = StatsSerie::new(&current_y).get_stats(aggregator);
                        aggregated_serie.push((current_x, stats.value as f32));
                    }
                    current_x = *x;
                    current_y = Vec::new();
                    current_y.push(*y);
                }
            }
            if current_y.len() != 0 {
                // calculate the metrics
                let stats = StatsSerie::new(&current_y).get_stats(aggregator);
                aggregated_serie.push((current_x, stats.value as f32));

            }
            // replace the serie with the aggregated one
            aggregated_data.insert(key.clone(), aggregated_serie);
        }
        Ok(aggregated_data.into())
    }

    pub fn get_range(&self) -> (Range<f32>, Range<f32>) {
        let mut y_min = f32::MAX;
        let mut y_max = f32::MIN;
    
        let mut x_min = f32::MAX;
        let mut x_max = f32::MIN;
    
        // parcour the data and get the min and max of each axis
        for (_, serie) in self.data.iter() {
            for (x, y) in serie.iter() {
                if *x < x_min {
                    x_min = *x;
                }
                if *x > x_max {
                    x_max = *x;
                }
    
                if *y < y_min {
                    y_min = *y;
                }
                if *y > y_max {
                    y_max = *y;
                }
            }
        }
    
        if y_min == f32::MAX || y_max == f32::MIN {// if the data is empty
            y_min = 0.0;
            y_max = 1.0;
        }
    
        if x_min == f32::MAX || x_max == f32::MIN {// if the data is empty
            x_min = 0.0;
            x_max = 1.0;
        }
    
        if x_min == x_max {
            x_min -= 0.5;
            x_max += 0.5;
        }
    
        (x_min..x_max, y_min..y_max)
    }

    pub fn get_data(&self) -> &HashMap<String, Vec<Point>> {
        &self.data
    }
}

impl From<HashMap<String, Vec<Point>>> for PlotSeries {
    fn from(data : HashMap<String, Vec<Point>>) -> Self {
        Self {
            data,
        }
    }
}

impl Into<HashMap<String, Vec<Point>>> for PlotSeries {
    fn into(self) -> HashMap<String, Vec<Point>> {
        self.data
    }
}


/// define a layout for the subplots
pub struct Layout {
    pub height : usize,
    pub width : usize,
}

impl Layout {
    pub fn new(width : usize, height : usize) -> Self {
        Self {
            height,
            width,
        }
    }
    /// get the plotter layout (row, col)
    pub fn get_plotter_layout(&self) -> (usize, usize) {
        (self.height, self.width)
    }

    pub fn get_nb_of_subplots(&self) -> usize {
        self.height * self.width
    }
}
