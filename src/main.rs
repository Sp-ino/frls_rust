use core::panic;
use matfile::MatFile;
use plotters::prelude::*;



fn plot_trace(curve: Vec<(f64, f64)>, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let root = BitMapBackend::new(path, (840, 680))
                                                .into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
                // Set the caption of the chart
                .caption("Sample trace", ("sans-serif", 40).into_font())
                // Set the size of the label region
                .x_label_area_size(30)
                .y_label_area_size(60)
                // Finally attach a coordinate on the drawing area and make a chart context
                .build_cartesian_2d(0.0f64..0.8e-5f64, -0.3e5f64..0.4e5f64)?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .draw()?;

    chart
        .draw_series(LineSeries::new(curve,
                                &BLUE))?;

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;

    Ok(())
}



fn get_data(filename: &str) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    let file = std::fs::File::open(filename)
            .expect("Could not open file!");

    let file = MatFile::parse(file)
            .expect("Could not parse .mat file");

    for arr in file.arrays() {
        print!("Found array named {:?}\n", arr.name());
    }
    let t = file.find_by_name("t").unwrap().data();
    let idata = file.find_by_name("idata").unwrap().data();
    let odata = file.find_by_name("odata2").unwrap().data();

    let time;
    let input;
    let target;

    if let matfile::NumericData::Double { real, imag: _ } = t {
        time = real.clone();
    } else {
        panic!("Could not convert");
    }

    if let matfile::NumericData::Double { real, imag: _ } = odata {
        input = real.clone();
    } else {
        panic!("Could not convert");
    }

    if let matfile::NumericData::Double { real, imag: _ } = idata {
        target = real.clone();
    } else {
        panic!("Could not convert");
    }

    (time, input, target)
}



fn main() -> Result<(), Box<dyn std::error::Error>> {
    // -----------------------------------------------------------------------------
    let filename = "data/all_data_for_VS_a.mat";

    let mut curve = Vec::new();

    let (t, input, targ) = get_data(filename);

    for (i, (px, py)) in t.iter().zip(targ).enumerate() {
        // print!("{} {}", px, py);
        let px = *px;
        let py = py;

        curve.push((px, py));
        // if i < 2000 {
        // }
    }
    // -----------------------------------------------------------------------------


    // -----------------------------------------------------------------------------
    plot_trace(curve, "sample_plot.png")?;
    // -----------------------------------------------------------------------------

    Ok(())
}
