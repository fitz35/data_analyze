use std::collections::HashMap;
use std::fmt::Display;

use crate::stats::get_outliers;
use crate::stats::stats_serie::{MetricName, StatsSerie};

use super::filtering::{Filter, Mask};
use super::plot_data::PlotSeries;

/// Define a trait for a key of a serie
pub trait SerieKey : Eq + std::hash::Hash + Copy + Display + Ord{
    /// get the display name of the serie
    fn get_display_name(&self) -> String;

    /// if the serie is numeric
    fn is_numeric(&self) -> bool;

    /// if the serie is a string
    fn is_string(&self) -> bool;

    /// if the serie is an object
    fn is_object(&self) -> bool;
}


/// Define a fully initialized key for a serie
/// Warn : must be called only once by file
/// Need the dependances :
/// ```
/// use serde_derive::{Deserialize, Serialize};
/// use std::fmt::{Display, Formatter};
/// use plot_helper::data::plottable::SerieKey;
/// use plot_helper::generate_plot_key;
/// generate_plot_key!(
///     MultiLineQueryKey[
///         ParsingTime { "parsing time (s)", Numeric},
///         ParsingMaxMemory { "parsing max memory (Mb)", Numeric }
///     ],
///     SingleLineQueryKey[
///         ParsingTime { "parsing time (s)", Numeric },
///         File { "file", String }
///     ]
/// );
/// assert_eq!(MultiLineQueryKey::ParsingTime.get_display_name(), "parsing time (s)");
/// assert_eq!(MultiLineQueryKey::ParsingTime.is_numeric(), true);
/// ```
#[macro_export]
macro_rules! generate_plot_key {
    ($($key_name:ident [ 
        $($variant:ident {
             $description:literal, $key_type:ident
        }),* 
    ]),+) => {
        /// Define the type of the key
        #[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
        enum KeyType {
            /// The key is numeric
            Numeric,
            /// The key is a string
            String,
            /// The key is an object
            Object
        }

        /// Define the keys

        $(
            #[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Copy)]
            pub enum $key_name {
                $($variant),*
            }

            impl SerieKey for $key_name {
                fn get_display_name(&self) -> String {
                    match self {
                        $($key_name::$variant => $description.to_string()),*
                    }
                }
                fn is_numeric(&self) -> bool {
                    match self {
                        $($key_name::$variant => KeyType::$key_type == KeyType::Numeric),*
                    }
                }
                fn is_string(&self) -> bool {
                    match self {
                        $($key_name::$variant => KeyType::$key_type == KeyType::String),*
                    }
                }
                fn is_object(&self) -> bool {
                    match self {
                        $($key_name::$variant => KeyType::$key_type == KeyType::Object),*
                    }
                }
            }

            impl Display for $key_name {
                fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{}", self.get_display_name())
                }
            }
            
            impl PartialOrd for $key_name {
                fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                    self.get_display_name().partial_cmp(&other.get_display_name())
                }
            }
            
            impl Ord for $key_name {
                fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                    self.get_display_name().cmp(&other.get_display_name())
                }
            }
        )*
    };
}

/// Define a trait for plottable data, indexed with a key
/// WARN : all the data must be the same size
pub trait Plottable<Key> where Key : SerieKey{

    /// Get the series of data (as f32) to plot, associated to the given key
    fn get_numeric_series(&self, key : &Key) -> Vec<f32>;

    /// Get the series of data (as string) to plot, associated to the given key
    fn get_string_series(&self, key : &Key) -> Vec<String>;

    /// Get the number of data for a key
    fn get_number_of_samples(&self) -> usize;

    /// aggregate the data and sort each point by legend
    /// Apply also the filters and remove the outliers
    /// NOTE : the data is aggregated by the x_serie_key if aggregation_metrics is Some
    fn aggregate(&self, 
        x_serie_key : &Key,
        y_serie_key : &Key,
        filters : &Option<Vec<&Filter<Key>>>,
        legends : &Vec<String>,
        remove_outliers : &Option<Vec<Key>>,
        aggregation_metrics : Option<MetricName>,
    ) -> Result<PlotSeries, Box<dyn std::error::Error>> 
    where
        Self : Sized
    {
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
        filters : &Option<Vec<&Filter<Key>>>,
        remove_outliers : &Option<Vec<Key>>,
    ) -> Mask 
    where 
        Self : Sized,
    {
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
        stats_serie_keys : &Vec<Key>, 
        sort_value_key : &Key
    ) -> HashMap<String, HashMap<Key, StatsSerie>>
    {
        let mut serie_by_key:HashMap<Key, HashMap<String, StatsSerie>>  = HashMap::new();
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


#[cfg(test)]
mod tests {
    use serde_derive::{Deserialize, Serialize};
    use std::fmt::{Display, Formatter};
    use crate::data::plottable::SerieKey;
    #[test]
    fn test_generate_plot_key() {
        generate_plot_key!(TestKey[
            A { "A", Numeric },
            B { "B", String },
            C { "C", Object }
        ]);

        let key = TestKey::A;
        assert_eq!(key.get_display_name(), "A");
        assert_eq!(key.is_numeric(), true);
        assert_eq!(key.is_string(), false);
        assert_eq!(key.is_object(), false);

        let key = TestKey::B;
        assert_eq!(key.get_display_name(), "B");
        assert_eq!(key.is_numeric(), false);
        assert_eq!(key.is_string(), true);
        assert_eq!(key.is_object(), false);

        let key = TestKey::C;

        assert_eq!(key.get_display_name(), "C");
        assert_eq!(key.is_numeric(), false);
        assert_eq!(key.is_string(), false);
        assert_eq!(key.is_object(), true);
    }



}