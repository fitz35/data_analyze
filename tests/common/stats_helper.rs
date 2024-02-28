use std::fmt::{Display, Formatter};

use plot_helper::stats::calculate_q1_q3;
use plot_helper::stats::stats_serie::{MetricName, StatsSerie};
use serde_derive::{Deserialize, Serialize};

use crate::common::ABSOLUTE_ERROR;



/// represent a test serie
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TestSerie {
    pub name: String,
    pub data: Vec<f32>,
    pub mean: Option<f64>,
    pub median: Option<f64>,
    pub q_1: Option<f32>,
    pub q_3: Option<f32>,
}

impl TestSerie {
    pub fn new(file_path : &str) -> Self {
        let data = std::fs::read_to_string(file_path).unwrap();
        serde_json::from_str(data.as_str()).unwrap()
    }

    pub fn new_from_dir(dir_path : &str) -> Vec<Self> {
        let mut res = Vec::new();
        for entry in std::fs::read_dir(dir_path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() {
                res.push(Self::new(path.to_str().unwrap()));
            }
        }
        res
    }
}

impl Display for TestSerie {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(self).unwrap())
    }
}


/// test the stats of the given instance
pub fn test_stats(instance : &TestSerie, verbose : bool) {
    let instance_name = if !verbose {
        format!("{} ", instance.name)
    }else{
        format!("{} ", instance)
    };
    let stats_serie = StatsSerie::new(&instance.data);
    let (q1, q3) = calculate_q1_q3(&instance.data);

    if instance.mean.is_none() {
        assert!(stats_serie.get_stats(MetricName::Mean).value.is_nan(), "The mean is not correct for the serie {} (Must be a NAN value).", instance_name);
    }else{
        let result = relative_eq!(stats_serie.get_stats(MetricName::Mean).value, instance.mean.unwrap(), epsilon = ABSOLUTE_ERROR);
        assert!(result, "The mean is not correct for the serie {}, with an obtained value of {} and an expected value of {}", instance_name, stats_serie.get_stats(MetricName::Mean).value, instance.mean.unwrap());
    }

    if instance.median.is_none() {
        assert!(stats_serie.get_stats(MetricName::Median).value.is_nan(), "The median is not correct for the serie {} (Must be a NAN value).", instance_name);
    }else{
        let result = relative_eq!(stats_serie.get_stats(MetricName::Median).value, instance.median.unwrap(), epsilon = ABSOLUTE_ERROR);
        assert!(result, "The median is not correct for the serie {}, with an obtained value of {} and an expected value of {}", instance_name, stats_serie.get_stats(MetricName::Median).value, instance.median.unwrap());
    }

    if instance.q_1.is_none() {
        assert!(q1.is_nan(), "The q1 is not correct for the serie {} (Must be a NAN value).", instance_name);
    }else{
        let result = relative_eq!(q1, instance.q_1.unwrap(), epsilon = ABSOLUTE_ERROR as f32);
        assert!(result, "The q1 is not correct for the serie {} with an obtained value of {} and an expected value of {}", instance_name, q1, instance.q_1.unwrap());
    }

    if instance.q_3.is_none() {
        assert!(q3.is_nan(), "The q3 is not correct for the serie {} (Must be a NAN value).", instance_name);
    }else{
        let result = relative_eq!(q3, instance.q_3.unwrap(), epsilon = ABSOLUTE_ERROR as f32);
        assert!(result, "The q3 is not correct for the serie {} with an obtained value of {} and an expected value of {}", instance_name, q3, instance.q_3.unwrap());
    }
}
