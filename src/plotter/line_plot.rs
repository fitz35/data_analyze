use std::collections::HashMap;
use std::path::Path;

use plotters::backend::BitMapBackend;
use plotters::chart::ChartBuilder;
use plotters::drawing::IntoDrawingArea;
use plotters::series::LineSeries;
use plotters::style::{Color, IntoFont, Palette, PaletteColor, WHITE};

use crate::data::filtering::Filter;
use crate::data::plot_data::Layout;
use crate::data::plottable::key::SerieKey;
use crate::data::plottable::Plottable;
use crate::params::{FIGURE_CAPTION_FONT_SIZE, LABEL_HORIZONTAL_SIZE, ONE_FIG_SIZE};
use crate::stats::stats_serie::MetricName;

use super::utils::{write_legend, CustomPalette};







/// plot the given data as a line
/// take a list of series to plot, to the format (x_serie_key, y_serie_key, filter)
/// If filter is Some, the data will be filtered by the given key and the given function (true to keep the data))
/// NOTE : the number of series to plot must be equal to the number of subplots
/// NOTE : If remove_outliers is Some, the outliers will be removed from the data with the given key
/// NOTE : The aggregation_metrics is the metric used to aggregate the data with the same x value
pub fn line_plot<Key>(
    data : &dyn Plottable<Key>, 
    legend_serie_key : Option<Key>,
    save_path : &str,
    layout : &Layout,

    series : Vec<(Key, Key, Option<Vec<&Filter<Key>>>)>,
    
    remove_outliers : Option<Vec<Key>>,
    aggregation_metric : MetricName,
) -> Result<(), Box<dyn std::error::Error>> 
where
    Key : SerieKey,
{
    if series.len() != layout.get_nb_of_subplots() {
        panic!("The number of series to plot ({}) is not equal to the number of subplots ({})", series.len(), layout.get_nb_of_subplots());
    }
    

    // initialise the plotter
    let image_path_o = Path::new(save_path);
    // (w, h)
    let global_size = (layout.width as u32 * ONE_FIG_SIZE.0 + LABEL_HORIZONTAL_SIZE, layout.height as u32 * ONE_FIG_SIZE.1);

    // global drawing
    let root_drawing_area = BitMapBackend::new(image_path_o, global_size).into_drawing_area();
    root_drawing_area.fill(&WHITE)?;
    // isolate the label area
    let (chart_drawing_area, label_drawing_area) = 
        root_drawing_area.split_horizontally(global_size.0 - LABEL_HORIZONTAL_SIZE);

    // get the drawing area for each subplot (row, col)
    let child_drawing_areas = chart_drawing_area.split_evenly(layout.get_plotter_layout());
    

    // get the legends
    let legends = 
        if legend_serie_key.is_some() {
            let legend_serie_key = legend_serie_key.unwrap();
            if legend_serie_key.is_string() {
                data.get_string_series(&legend_serie_key)
            } else {
                data.get_numeric_series(&legend_serie_key).iter().map(|v| v.to_string()).collect()
            }
        }else{
            vec!["all".to_string(); data.get_number_of_samples()]
        };
    // associate each legend to a color
    let mut legend_to_color : HashMap<String, PaletteColor<CustomPalette>> = HashMap::new();
    let mut legend_index = 0;

    // plot each serie
    for (_, (
            (x_serie_key, y_serie_key, filters), root)
        )
    in series.iter().zip(child_drawing_areas.iter()).enumerate() {
        // group the data by legend
        let grouped_data = 
            data.aggregate(x_serie_key, y_serie_key, filters, &legends, &remove_outliers, Some(aggregation_metric))?;
        
        let (range_x, range_y) = grouped_data.get_range();

        // define the chart

        let caption = format!("{} per {}", x_serie_key.get_display_name(), y_serie_key.get_display_name());
        let mut chart = ChartBuilder::on(&root)
            .caption(caption.as_str(), ("sans-serif", FIGURE_CAPTION_FONT_SIZE).into_font())
            .margin(5)
            .x_label_area_size(40)
            .y_label_area_size(60)
            .build_cartesian_2d(range_x, range_y)?;

        chart.configure_mesh()
            .x_desc(x_serie_key.get_display_name().as_str())
            .y_desc(y_serie_key.get_display_name().as_str())
            .x_label_formatter(&|x| format!("{:+e}", x))
            .y_label_formatter(&|y| format!("{:+e}", y))
            .draw()?;


        // plot the data
        for (legend, data_for_legend) in grouped_data.get_data().into_iter() {
            // update the legend color
            if !legend_to_color.contains_key(legend.to_string().as_str()) { 
                legend_to_color.insert(legend.to_string(), CustomPalette::pick(legend_index));// loop over the palette
                legend_index += 1;
            }
            let color = legend_to_color.get(legend).unwrap();
            chart
                .draw_series(
                    LineSeries::new(
                        data_for_legend.iter().map(|(x, y)| (*x, *y)),
                        color.filled().stroke_width(1),
                    )
                )?;
        }
    }// end of for each serie

    write_legend(&label_drawing_area, &legend_to_color, &legend_serie_key)?;

    root_drawing_area.present()?;


    Ok(())
}