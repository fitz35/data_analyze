use std::ops::AddAssign;

use serde_derive::{Deserialize, Serialize};



/// this a a class which handle the correctness stats
/// NOTE : To save it in a file, use the csv format and the two methods get_headers and get_values
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorrectnessStatsWrapper {
    total_sample: u64,
    true_positive: u64,
    true_positive_proportion: f64,
    false_positive: u64,
    false_positive_proportion: f64,
    false_negative: u64,
    false_negative_proportion: f64,
    true_negative: u64,
    true_negative_proportion: f64,
    recall: f64,
    precision: f64,
    f1: f64,
    accuracy: f64,
}

impl Default for CorrectnessStatsWrapper {
    fn default() -> Self {
        CorrectnessStatsWrapper {
            total_sample: 0,
            true_positive: 0,
            true_positive_proportion: 0.0,
            false_positive: 0,
            false_positive_proportion: 0.0,
            false_negative: 0,
            false_negative_proportion: 0.0,
            true_negative: 0,
            true_negative_proportion: 0.0,
            recall: 0.0,
            precision: 0.0,
            f1: 0.0,
            accuracy: 0.0,
        }
    }
}

impl CorrectnessStatsWrapper {
    pub fn new(
        true_positive: u64,
        false_positive: u64,
        false_negative: u64,
        true_negative: u64,
    ) -> CorrectnessStatsWrapper {
        let total_sample = true_positive + false_positive + false_negative + true_negative;
        let recall = true_positive as f64 / (true_positive + false_negative) as f64;
        let precision = true_positive as f64 / (true_positive + false_positive) as f64;
        let f1 = 2.0 * (precision * recall) / (precision + recall);
        let accuracy = (true_positive + true_negative) as f64 / (true_positive + true_negative + false_positive + false_negative) as f64;
        CorrectnessStatsWrapper {
            total_sample: total_sample,
            true_positive,
            true_positive_proportion: true_positive as f64 / total_sample as f64,
            false_positive,
            false_positive_proportion: false_positive as f64 / total_sample as f64,
            false_negative,
            false_negative_proportion: false_negative as f64 / total_sample as f64,
            true_negative,
            true_negative_proportion: true_negative as f64 / total_sample as f64,
            recall,
            precision,
            f1,
            accuracy,
        }
    }

    pub fn get_headers() -> &'static [&'static str ; 9] {
        &[
            "total sample",
            "true_positive",
            "false_positive",
            "false_negative",
            "true_negative",
            "recall",
            "precision",
            "f1",
            "accuracy"
        ]
    }

    pub fn get_values(&self) -> [String ; 9] {
        [
            self.total_sample.to_string(),
            format!("{} ({:.0}%)", self.true_positive, self.true_positive_proportion * 100.0),
            format!("{} ({:.0}%)", self.false_positive, self.false_positive_proportion * 100.0),
            format!("{} ({:.0}%)", self.false_negative, self.false_negative_proportion * 100.0),
            format!("{} ({:.0}%)", self.true_negative, self.true_negative_proportion * 100.0),
            format!("{:.2}", self.recall),
            format!("{:.2}", self.precision),
            format!("{:.2}", self.f1),
            format!("{:.2}", self.accuracy),
        ]
    }

    pub fn get_total_sample(&self) -> u64 {
        self.total_sample
    }

    pub fn get_true_positive(&self) -> u64 {
        self.true_positive
    }

    pub fn get_true_positive_proportion(&self) -> f64 {
        self.true_positive_proportion
    }

    pub fn get_false_positive(&self) -> u64 {
        self.false_positive
    }

    pub fn get_false_positive_proportion(&self) -> f64 {
        self.false_positive_proportion
    }

    pub fn get_false_negative(&self) -> u64 {
        self.false_negative
    }

    pub fn get_false_negative_proportion(&self) -> f64 {
        self.false_negative_proportion
    }

    pub fn get_true_negative(&self) -> u64 {
        self.true_negative
    }

    pub fn get_true_negative_proportion(&self) -> f64 {
        self.true_negative_proportion
    }

    pub fn get_recall(&self) -> f64 {
        self.recall
    }

    pub fn get_precision(&self) -> f64 {
        self.precision
    }

    pub fn get_f1(&self) -> f64 {
        self.f1
    }

    pub fn get_accuracy(&self) -> f64 {
        self.accuracy
    }

    pub fn ref_add(&self, other: &CorrectnessStatsWrapper) -> CorrectnessStatsWrapper {
        CorrectnessStatsWrapper::new(
            self.true_positive + other.true_positive,
            self.false_positive + other.false_positive,
            self.false_negative + other.false_negative,
            self.true_negative + other.true_negative,
        )
    }
}

impl AddAssign for CorrectnessStatsWrapper {
    fn add_assign(&mut self, other: CorrectnessStatsWrapper) {
        *self = self.ref_add(&other);
    }
}