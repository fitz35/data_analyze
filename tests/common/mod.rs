pub mod stats_helper;
pub mod plottable_helper;

// ------------- tests params -------------
pub const STATS_RESSOURCES_DIR_PATH : &'static str = "tests/ressources/stats_series/data/";

pub const ABSOLUTE_ERROR : f64 = 0.0001;


pub const PYTHON_STATS_EXE_PATH : &'static str = "tests/ressources/stats_series/generate_stats_series.py";


pub fn get_stats_size_to_test() -> Vec<u32> {
    vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 20, 30]
}


// ------------------------------ stats helper ------------------------------

