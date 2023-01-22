use core::panic;
use matfile::MatFile;
use plotters::prelude::*;
use std::collections::HashMap;



pub fn get_matfile_data<'a>(matfile: &'a MatFile) -> HashMap<&'a str, Vec<f64>> {
    // Gets data from matfile

    let mut map = HashMap::new();

    for (idx, arr) in matfile.arrays().iter().enumerate() {
        let arr_name = arr.name();

        
        let numdata = matfile
        .find_by_name(arr_name)
        .unwrap_or_else(|| panic!("{} field is None!", arr_name))
        .data();
        
        if let matfile::NumericData::Double { real, imag: _ } = numdata {
            let vec = real.clone();
            print!("Array at index {} is named {} and has length {}\n", idx, arr_name, vec.len());
            map.insert(arr_name, vec);
        } else {
            panic!("Could not convert vector {} to Vec", arr_name);
        }
    }

    map
}



pub fn plot_curve_2d(curve: Vec<(f64, f64)>, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Plots a 2d curve represented as an array of (f64, f64).

    let root = BitMapBackend::new(path, (840, 680))
                                            .into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
                // Set the caption of the chart
                .caption("Sample trace", ("sans-serif", 20).into_font())
                // Set the size of the label region
                .x_label_area_size(30)
                .y_label_area_size(60)
                // Finally attach a coordinate on the drawing area and make a chart context
                .build_cartesian_2d(0.0f64..0.8e-5f64, -0.3e5f64..0.3e5f64)?;

    chart
        .configure_mesh()
        .disable_mesh()
        .draw()?;

    chart
        .draw_series(LineSeries::new(curve,
                                &BLUE))?;


    root.present()?;

    Ok(())
}



pub fn curve_2d(x: Vec<f64>, y: Vec<f64>, maxlen: Option<usize>) -> Vec<(f64, f64)> {
    // Generates a vector of (f64, f64) from two vectors of f64
    // Useful for plotting with the `plotters` crate.

    if x.len() != y.len() {
        panic!("curve_2d: x and y must have the same length!");
    }

    let max_length = maxlen.unwrap_or(x.len());
    let mut curve = Vec::new();

    for (i, (px, py)) in x.iter().zip(y).enumerate() {
        let px = *px;
        let py = py;

        if i < max_length {
            curve.push((px, py));
        }
    }

    curve
}