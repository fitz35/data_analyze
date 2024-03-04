use std::collections::HashMap;

use plotters::backend::BitMapBackend;
use plotters::chart::{ChartBuilder, SeriesLabelPosition};
use plotters::coord::Shift;
use plotters::drawing::DrawingArea;
use plotters::element::Circle;
use plotters::style::{Color, Palette, PaletteColor, BLACK, WHITE};

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
        format!("{:.2}", x)
    }
}