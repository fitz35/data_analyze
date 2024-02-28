use std::collections::HashMap;
use std::fs;

use plot_helper::data::plot_data::PlotSeries;
use plot_helper::stats::stats_serie::MetricName;


mod common;

#[macro_use]
extern crate approx;

/// organise the data with the number of sample as the legend, and each name as 
/// the x coordinate, in the order of the TestSeries output
fn get_all_test_plot_series_from_dir() -> Result<(PlotSeries, HashMap<String, Vec<common::stats_helper::TestSerie>>), std::io::Error> {
    let mut all_plot_serie = PlotSeries::new();
    let mut all_test_series = HashMap::new();

    let entries = fs::read_dir(common::STATS_RESSOURCES_DIR_PATH)?;

    // for each directory in the ressources directory, get all the serie
    for entry in entries {
        let entry = entry?;
        let dir_path = entry.path();

        if dir_path.is_dir() {
            let dir_test_series = common::stats_helper::TestSerie::new_from_dir(dir_path.to_str().unwrap());
            
            let mut legend : Option<String> = None;
            let mut x_coordinate = 0.0_f32;
            for test_serie in dir_test_series.iter() {
                if legend.is_none() {
                    legend = Some(format!("{}", test_serie.data.len()));
                }

                for data in test_serie.data.iter() {
                    all_plot_serie.add(legend.clone().unwrap(), (x_coordinate, *data));
                }

                x_coordinate += 1.0;
            }

            if legend.is_some(){ // consume legend
                all_test_series.insert(legend.unwrap(), dir_test_series);
            }
        }
    }

    Ok((all_plot_serie, all_test_series))

}


#[test]
fn apply_aggregator_test() -> Result<(), Box<dyn std::error::Error>> {
    let (all_plot_serie, all_test_series) = get_all_test_plot_series_from_dir()?;

    // mean
    {
        let mean_plot_serie = all_plot_serie.apply_aggregator(MetricName::Mean)?;
        let keys = all_test_series.keys().collect::<Vec<&String>>();

        for legend in keys.iter() {
            let test_series = all_test_series.get(*legend).unwrap();
            let points = mean_plot_serie.get_data().get(*legend).unwrap();

            for point in points.iter() {
                let test_serie = test_series.get(point.0 as usize).unwrap();// get the test serie at the x coordinate
                if test_serie.mean.is_none() {
                    unreachable!("The mean is not defined for the test serie {}, this doesn't mean to arrive....The vector mus be empty.", test_serie.name);
                }
                assert_relative_eq!(test_serie.mean.unwrap(), point.1 as f64, max_relative = common::ABSOLUTE_ERROR);
            }
        }
        
    }




    Ok(())
}