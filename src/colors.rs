use crate::vector3::Vector3 as Color;
use anyhow::Context;
use std::fs::File;
use std::io::Write;

pub fn write_color(file: &mut File, color: &Color) -> anyhow::Result<()> {
    let integer_red = (255.999 * color[0]) as u8;
    let integer_green = (255.999 * color[1]) as u8;
    let integer_blue = (255.999 * color[2]) as u8;

    // write rgb value to file
    file.write(format!("{} {} {}\n", integer_red, integer_green, integer_blue).as_bytes())
        .with_context(|| "Issue writing to file".to_string())?;

    Ok(())
}
