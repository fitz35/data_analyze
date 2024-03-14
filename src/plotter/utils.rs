use std::collections::HashMap;

use plotters::backend::BitMapBackend;
use plotters::chart::{ChartBuilder, SeriesLabelPosition};
use plotters::coord::Shift;
use plotters::drawing::DrawingArea;
use plotters::element::Circle;
use std::ops::Range;
use plotters::style::{Color, Palette, PaletteColor, RGBColor, BLACK, WHITE};

use crate::data::linspace::Linspace;
use crate::data::plottable::key::SerieKey;



pub struct CustomPalette;


impl Palette for CustomPalette {
    const COLORS: &'static [(u8, u8, u8)] = &[
        (230, 25, 75),
        (60, 180, 75),
        (255, 225, 25),
        (0, 130, 200),
        (245, 130, 48),
        (145, 30, 180),
        (70, 240, 240),
        (240, 50, 230),
        (210, 245, 60),
        (250, 190, 190),
        (0, 128, 128),
        (230, 190, 255),
        (170, 110, 40),
        (255, 250, 200),
        (128, 0, 0),
        (170, 255, 195),
        (128, 128, 0),
        (255, 215, 180),
        (0, 0, 128),
        (128, 128, 128),
        (0, 0, 0),
    ];
}





/// draw the legend on the given drawing area
pub fn write_legend<Key> (
    label_drawing_area: &DrawingArea<BitMapBackend<'_>, Shift>,
    legend_to_color : &HashMap<String, PaletteColor<CustomPalette>>,
    legend_serie_key : &Option<Key>
) -> Result<(), Box<dyn std::error::Error>>
where 
    Key : SerieKey,
{
    // draw the legend on a fantome chart
    let mut label_chart = 
    ChartBuilder::on(label_drawing_area)
    .margin(5)
    .build_cartesian_2d(0..1, 0..1)?;

    label_chart
        .configure_mesh()
        .disable_mesh()
        .draw()?;




    let dummy_data : Vec<(i32, i32)> = Vec::new();

    // draw phantome serie to get the legend

    // begin by the first one indicating the key used
    if let Some(legend_serie_key) = legend_serie_key {
        let color = RGBColor(255, 255, 255).mix(0.0);
        
        let serie_unlabellized = label_chart
                .draw_series(
                    dummy_data.iter()
                        .map(|(x, y)| Circle::new((*x, *y), 2, color.filled())),
            )?;
        serie_unlabellized.label(legend_serie_key.get_display_name()).legend(move |(x, y)| {
            Circle::new((x, y), 5, color.filled())
        });
    }

    
    let mut unique_legends = legend_to_color.keys().collect::<Vec<_>>();
    if legend_serie_key.is_some() && legend_serie_key.unwrap().is_numeric(){
        unique_legends.sort_by(|a, b| a.parse::<f32>().unwrap().partial_cmp(&b.parse::<f32>().unwrap()).unwrap());
    }

    for legend in unique_legends.iter() {
        // skip empty legend
        if legend == &"" {
            continue;
        }
        let color = legend_to_color.get(*legend).unwrap();
        let serie_unlabellized = label_chart
                .draw_series(
                    dummy_data.iter()
                        .map(|(x, y)| Circle::new((*x, *y), 2, color.filled())),
            )?;
            serie_unlabellized.label(legend.to_string()).legend(move |(x, y)| {
                Circle::new((x, y), 5, color.filled())
            });
    }

    label_chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .position(SeriesLabelPosition::MiddleMiddle)
        .draw()?;

    Ok(())
}

const TRESHOLD : f32 = 1000.0;

/// Format a number to a string
pub(crate) fn axe_number_formater(x: &f32) -> String {
    let x = *x;
    if x < -TRESHOLD || (x > -1.0/TRESHOLD && x < 1.0/TRESHOLD) || x > TRESHOLD {
        format!("{:.0e}", x)
    } else {
        format!("{:.3}", x)
    }
}

const SERIE_DIVISION : usize = 500;

/// The goal of the function is to compress a dataserie to accelerate the plotting
/// it will cut the graph into piece en make the average of each piece
/// NOTE : The function take the borow of the data to avoid the memory allocation
pub(crate) fn compress_data_serie(to_compress : Vec<(f32, f32)>, range_x : &Range<f32>, range_y : &Range<f32>) -> Vec<(f32, f32)> {
    // cut the range into SERIE_DIVISION pieces
    let x_linspace = Linspace::new(range_x.start as f64, range_x.end as f64, SERIE_DIVISION);
    let y_linspace = Linspace::new(range_y.start as f64, range_y.end as f64, SERIE_DIVISION);

    // create the hashmap to store the data
    // the key is the index of the piece in the x and y linspace
    // the value is a tuple with the sum of the x and y and the number of points
    // use welfort's algorithm
    let mut result_map : HashMap<(usize, usize), (f32, f32, usize)> = HashMap::new();
    for (x, y) in to_compress.into_iter() {
        let x_index = x_linspace.index_of(&(x as f64)).unwrap();
        let y_index = y_linspace.index_of(&(y as f64)).unwrap();
        let entry = result_map.entry((x_index, y_index)).or_insert((0.0, 0.0, 0));
        entry.2 += 1;
        entry.0 += (x - entry.0) / entry.2 as f32;
        entry.1 += (y - entry.1) / entry.2 as f32;
        
    }

    // create the result vector
    let mut result : Vec<(f32, f32)> = Vec::new();
    for (x, y, _) in result_map.values() {
        result.push((*x, *y));
    }
    
    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_dataset() {
        let to_compress: Vec<(f32, f32)> = Vec::new();
        let range_x = Range { start: 0.0, end: 10.0 };
        let range_y = Range { start: 0.0, end: 10.0 };

        let compressed = compress_data_serie(to_compress, &range_x, &range_y);

        assert_eq!(compressed.len(), 0);
    }
}