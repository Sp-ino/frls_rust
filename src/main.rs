use core::panic;
use matfile::MatFile;
use plotters::prelude::*;




fn main() -> Result<(), Box<dyn std::error::Error>> {
    // -----------------------------------------------------------------------------
    let filename = "data/all_data_for_VS_a.mat";
    let file = std::fs::File::open(filename)
                        .expect("Could not open file!");

    let data = MatFile::parse(file)
                            .expect("Could not parse .mat file");

                            for arr in data.arrays() {
        print!("Found array named {:?}\n", arr.name());
    }
    let t = data.find_by_name("t").unwrap().data();
    let idata = data.find_by_name("idata").unwrap().data();

    // let x: ndarray::ArrayD<f64> = data.try_into()?;
    // let y: ndarray::ArrayD<f64> = data.try_into()?;

    let x;
    let y;

    if let matfile::NumericData::Double { real, imag: _ } = t {
        x = real;
    } else {
        panic!("Could not convert")
    }

    if let matfile::NumericData::Double { real, imag: _ } = idata {
        y = real;
    } else {
        panic!("Could not convert")
    }

    let mut curve = Vec::new();
    for (i, (px, py)) in x.iter().zip(y).enumerate() {
        // print!("{} {}", px, py);
        let px = *px;
        let py = *py;

        if i < 2000 {
            curve.push((px, py));
        }
    }
    // println!("length of curve: {}", curve.len());
    // println!("curve: {:?}", curve);

    // let curve = vec![(0.0, 0.0), (5.0, 5.0), (8.0, 7.0), (9.0, 7.0), (10.0, 6.0)];
    // -----------------------------------------------------------------------------


    // -----------------------------------------------------------------------------
    let root = BitMapBackend::new("graph.png", (840, 680))
                                                .into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
                // Set the caption of the chart
                .caption("Sample trace", ("sans-serif", 40).into_font())
                // Set the size of the label region
                .x_label_area_size(20)
                .y_label_area_size(40)
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
    // -----------------------------------------------------------------------------

}
