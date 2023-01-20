mod datautils;
use datautils::{get_matfile_data, plot_curve_2d, curve_2d};


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // -----------------------------------------------------------------------------
    let filename = "data/all_data_for_VS_a.mat";
    let (t, input, _targ) = get_matfile_data(filename);
    // -----------------------------------------------------------------------------

    // -----------------------------------------------------------------------------
    println!("{} {}", t.len(), input.len());
    let curve = curve_2d(t, input, None);
    plot_curve_2d(curve,  "sample_plot.png")?;
    // -----------------------------------------------------------------------------

    Ok(())
}
