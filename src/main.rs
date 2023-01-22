mod datautils;
use datautils::{get_matfile_data, plot_curve_2d, curve_2d};
use matfile::MatFile;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // -----------------------------------------------------------------------------
    let filename = "data/all_data_for_VS_a.mat";
    let file = std::fs::File::open(filename)
                                                .expect("Could not open file!");

    let matfile = MatFile::parse(file)
                                    .expect("Could not parse .mat file");

    let data  = get_matfile_data(&matfile);
    let time = data.get("t")
                            .unwrap_or_else(|| panic!("Field t is empty!"))
                            .clone();
    let _target = data.get("idata")
                            .unwrap_or_else(|| panic!("Field idata is empty!"))
                            .clone();
    let input = data.get("odata2")
                            .unwrap_or_else(|| panic!("Field odata2 is empty!"))
                            .clone();
    // -----------------------------------------------------------------------------

    // -----------------------------------------------------------------------------
    println!("{} {}", time.len(), input.len());
    let curve = curve_2d(time, input, None);
    plot_curve_2d(curve,  "sample_plot.png")?;
    // -----------------------------------------------------------------------------

    Ok(())
}
