use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[cfg(feature = "parrallelize")]
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

/// represent a serie with its stats
#[derive(Debug, Clone)]
pub struct StatsSerie {
    pub serie : Vec<f32>,
    pub stats : HashMap<MetricName, MetricValue>,
}

impl StatsSerie {
    pub fn new(serie : &Vec<f32>) -> Self {
        let mut stats = HashMap::new();

        if serie.len() == 0 {
            stats.insert(MetricName::Mean, MetricValue::mean(f64::NAN));
            stats.insert(MetricName::Median, MetricValue::median(f64::NAN));

            return Self {
                serie : serie.clone(),
                stats,
            }
        }

        #[cfg(not(feature = "parrallelize"))]
        stats.insert(
            MetricName::Mean, 
            MetricValue::mean(serie.iter().map(|f| *f as f64).sum::<f64>() / serie.len() as f64)
        );
        #[cfg(feature = "parrallelize")]
        stats.insert(
            MetricName::Mean, 
            MetricValue::mean(serie.par_iter().map(|f| *f as f64).sum::<f64>() / serie.len() as f64)
        );

        let sorted_serie = {
            let mut sorted_serie = serie.clone();
            sorted_serie.sort_by(|a, b| a.partial_cmp(b).unwrap());
            sorted_serie
        };
        if serie.len() % 2 == 0 {
            stats.insert(MetricName::Median, MetricValue::median((sorted_serie[serie.len() / 2] as f64 + sorted_serie[serie.len() / 2 - 1] as f64) / 2.0));
        }else{
            stats.insert(MetricName::Median, MetricValue::median(sorted_serie[serie.len() / 2] as f64));
        }

        Self {
            serie : serie.clone(),
            stats,
        }
    }

    pub fn get_stats(&self, metric : MetricName) -> MetricValue {
        self.stats.get(&metric).unwrap().clone()
    }
}

impl Display for StatsSerie {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut str = String::new();
        let mut keys = self.stats.keys().collect::<Vec<&MetricName>>();
        keys.sort();
        for key in keys {
            let value = self.stats.get(key).unwrap();
            str.push_str(&format!("{} ", value));
        }

        write!(f, "{}", str)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MetricValue {
    pub name : MetricName,
    pub value : f64,
}

impl Display for MetricValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {:.2}", self.name, self.value)
    }
}

impl MetricValue {
    pub fn median(value : f64) -> Self {
        Self {
            name : MetricName::Median,
            value,
        }
    }

    pub fn mean(value : f64) -> Self {
        Self {
            name : MetricName::Mean,
            value,
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub enum MetricName {
    Mean,
    Median,
}

impl Display for MetricName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_name())
    }
}


impl MetricName {
    pub fn get_name(&self) -> String {
        match self {
            MetricName::Mean => "mean".to_string(),
            MetricName::Median => "median".to_string(),
        }
    }
}