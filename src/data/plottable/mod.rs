use std::collections::HashMap;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use serde_derive::{Deserialize, Serialize};

use crate::data::plottable::key::SerieKey;
use crate::stats::get_outliers;
use crate::stats::stats_serie::{MetricName, StatsSerie};

use self::sample::{MultipleSample, Sample, SimpleSample};

use super::filtering::{Filter, Mask};
use super::plot_data::PlotSeries;

pub mod key;
pub mod sample;

/// Define method to be plottable by the plotter
pub trait Plottable<KeyType> 
where
    KeyType : SerieKey
{
    /// Get the number of samples
    fn get_number_of_samples(&self) -> usize;
    /// Get the numeric series associated to the given key
    fn get_numeric_series(&self, key : &KeyType) -> Vec<f32>;
    /// Get the string series associated to the given key
    fn get_string_series(&self, key : &KeyType) -> Vec<String>;

    /// aggregate the data and sort each point by legend
    /// Apply also the filters and remove the outliers
    /// NOTE : the data is aggregated by the x_serie_key if aggregation_metrics is Some
    fn aggregate(&self, 
        x_serie_key : &KeyType,
        y_serie_key : &KeyType,
        filters : &Option<Vec<&Filter<KeyType>>>,
        legends : &Vec<String>,
        remove_outliers : &Option<Vec<KeyType>>,
        aggregation_metrics : Option<MetricName>,
    ) -> Result<PlotSeries, Box<dyn std::error::Error>>{
        let mut aggregate_data = PlotSeries::new();

        // filter the data (filter and outliers)
        let filter_mask : Mask = self.combine_filter(filters, remove_outliers);

        let x_serie = filter_mask.apply(&self.get_numeric_series(x_serie_key));
        let y_serie = filter_mask.apply(&self.get_numeric_series(y_serie_key));
        let legends = filter_mask.apply(&legends);

        // zip the data
        let iterator = 
            x_serie.iter()
            .zip(y_serie.iter())
            .zip(legends.iter());

        // group the data and get the axes
        for ((x, y), legend) in iterator {
            aggregate_data.add(legend.clone(), (*x, *y));
        }

        // aggregate the data
        if let Some(metrics) = aggregation_metrics {
            aggregate_data = aggregate_data.apply_aggregator( metrics)?;
        }

        Ok(aggregate_data)
    }

    /// combine the filters with the remove_outliers, and return the combined mask
    fn combine_filter(
        &self,
        filters : &Option<Vec<&Filter<KeyType>>>,
        remove_outliers : &Option<Vec<KeyType>>,
    ) -> Mask{
        let mut filter_mask : Mask = Mask::new(self.get_number_of_samples());
        match filters {
            Some(filters_vec) => {
                for filter in filters_vec {
                    if filter.get_key().is_numeric() {
                        let serie = self.get_numeric_series(filter.get_key());
                        filter_mask.merge(&serie.iter().map(|v| filter.get_filter_number()(*v)).collect());
                    } else {
                        let serie = self.get_string_series(filter.get_key());
                        filter_mask.merge(&serie.iter().map(|v| filter.get_filter_str()(v)).collect());
                    }
                }
            },
            None => (),
        };

        if remove_outliers.is_some() {
            for key in remove_outliers.as_ref().unwrap() {
                let serie = self.get_numeric_series(&key);
                filter_mask.merge(&get_outliers(&serie).iter().map(|v| !v).collect()); // inverse, because we keep the value that are not outliers
            }
        };

        filter_mask
    }

    /// Collect statistics for multiple series sorted by a the uniquee value of a specified key.
    /// This function is optimized for speed but not for memory (O(n)).
    /// Warning: Avoid calling this function multiple times with different metrics as it may be slow.
    fn collect_stats_sorted_by_unique_values(
        &self, 
        stats_serie_keys : &Vec<KeyType>, 
        sort_value_key : &KeyType
    ) -> HashMap<String, HashMap<KeyType, StatsSerie>> {
        let mut serie_by_key:HashMap<KeyType, HashMap<String, StatsSerie>>  = HashMap::new();
        let sort_serie = if sort_value_key.is_numeric() {
            self.get_numeric_series(sort_value_key).iter().map(|f| f.to_string()).collect::<Vec<String>>()
        }else{
            self.get_string_series(sort_value_key)
        };
        for key in stats_serie_keys {
            assert!(key.is_numeric(), "stats serie key must be numeric");
            let mut sorted_series: HashMap<String, Vec<f32>> = HashMap::new();

            let key_series = self.get_numeric_series(key);
            let iter = key_series.iter().zip(sort_serie.iter());
            for (key_value, sort_value) in iter {
                let serie = sorted_series.entry(sort_value.to_string()).or_insert(Vec::new());
                serie.push(*key_value);
            }


            let sorted_stated_serie = sorted_series.iter().map(|(key, serie)| {
                let stats_serie = StatsSerie::new(&serie);
                (key.clone(), stats_serie)
            }).collect::<HashMap<String, StatsSerie>>();
            serie_by_key.insert(*key, sorted_stated_serie);
        }

        let mut serie_by_sort = HashMap::new();

        for (key, inner_map) in serie_by_key.iter() {
            for (inner_key, inner_value) in inner_map.iter() {
                serie_by_sort
                    .entry(inner_key.clone())
                    .or_insert_with(HashMap::new)
                    .insert(key.clone(), inner_value.clone());
            }
        }

        return serie_by_sort;
    }
}


/// Define a trait to regroup the samples into a plottable
pub trait PlottableSamples<SampleType, KeyType> 
where
    Self : Plottable<KeyType> + Sized,
    SampleType : Sample<KeyType>,
    KeyType : SerieKey
{
    /// Create a new PlottableSamples from a list of sample
    fn new(samples : Vec<SampleType>) -> Self;

    /// get the samples
    fn get_samples(&self) -> &Vec<SampleType>;
}

/// Define a mean to load the data from a list of file paths if this is a SimpleSample
pub trait PlottableSimpleSamplesFromPaths<SampleType, KeyType> 
where
    Self : PlottableSamples<SampleType, KeyType>,
    SampleType : SimpleSample<KeyType>,
    KeyType : SerieKey
{
    /// Create a new PlottableSamples from a list of file paths (async)
    fn new_async_from_paths(file_paths : &Vec<String>) -> Self {
        let samples = file_paths.par_iter().flat_map(
            |path| {
                if let Ok(sample) = SampleType::new_from_file_path(path) {
                    Some(sample)
                } else {
                    None
                }
            }
        ).collect::<Vec<SampleType>>();

        Self::new(samples)
    }
}

/// Define a mean to load the data from a list of file paths if this is a MultipleSample
pub trait PlottableMultipleSamplesFromPaths<SampleType, KeyType> 
where
    Self : PlottableSamples<SampleType, KeyType>,
    SampleType : MultipleSample<KeyType>,
    KeyType : SerieKey
{
    /// Create a new PlottableSamples from a list of file paths (async)
    fn new_async_from_paths(file_paths : &Vec<String>) -> Self {
        let samples = file_paths.par_iter().flat_map(
            |path| {
                if let Ok(samples) = SampleType::new_from_file_path(path) {
                    Some(samples)
                } else {
                    None
                }
            }
        ).flatten().collect::<Vec<SampleType>>();

        Self::new(samples)
    }
}

/// Define plottable data linked to a sample (wrapper for the sample data on Plottable)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlottableStruct<SampleType, KeyType>
where 
    KeyType : SerieKey,
    SampleType : Sample<KeyType>

{
    samples : Vec<SampleType>,
    _key_type : std::marker::PhantomData<KeyType>,
}

impl<SampleType, KeyType> 
    PlottableSimpleSamplesFromPaths<SampleType, KeyType>
for PlottableStruct<SampleType, KeyType>
where 
    KeyType : SerieKey,
    SampleType : SimpleSample<KeyType>
{

}

impl<SampleType, KeyType> 
    PlottableMultipleSamplesFromPaths<SampleType, KeyType>
for PlottableStruct<SampleType, KeyType>
where 
    KeyType : SerieKey,
    SampleType : MultipleSample<KeyType>
{

}

impl<SampleType, KeyType> 
    Plottable<KeyType>
for PlottableStruct<SampleType, KeyType>
where 
    KeyType : SerieKey,
    SampleType : Sample<KeyType>
{
    fn get_number_of_samples(&self) -> usize {
        self.samples.len()
    }

    fn get_numeric_series(&self, key : &KeyType) -> Vec<f32> {
        self.samples.iter().map(|sample| sample.get_numeric_value(key)).collect()
    }

    fn get_string_series(&self, key : &KeyType) -> Vec<String> {
        self.samples.iter().map(|sample| sample.get_string_value(key)).collect()
    }
}

impl <SampleType, KeyType> 
    PlottableSamples<SampleType, KeyType>
for PlottableStruct<SampleType, KeyType>
where 
    KeyType : SerieKey,
    SampleType : Sample<KeyType>
{
    fn new(samples : Vec<SampleType>) -> Self {
        Self {
            samples,
            _key_type : std::marker::PhantomData,
        }
    }

    fn get_samples(&self) -> &Vec<SampleType> {
        &self.samples
    }
}
