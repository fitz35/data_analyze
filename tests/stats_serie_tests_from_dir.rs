
mod common;

#[macro_use]
extern crate approx;



#[test]
fn test_stats_series_from_dir() {
    
    for instance in common::stats_helper::TestSerie::new_from_dir(common::STATS_RESSOURCES_DIR_PATH){
        common::stats_helper::test_stats(&instance, false);
    }
}