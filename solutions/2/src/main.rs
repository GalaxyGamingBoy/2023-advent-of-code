use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), std::io::Error> {
    // Read input file
    let file = File::open("../../inputs/2/input.in")?;
    let buf = BufReader::new(file);

    let mut total: u16 = 0;
    let mut total_power: u16 = 0;
    for ln in buf.lines() {
        // Parse the data from the file
        let data = ln?.replace("Game", "").replace(" ", "").replace(";", ",");
        let data = data.split(":").collect::<Vec<&str>>();

        let id = data[0].parse::<u16>().expect("");
        let cubes = data[1].split(",");

        let mut max_r: u16 = 0;
        let mut max_g: u16 = 0;
        let mut max_b: u16 = 0;

        // For each cube, sort by color and set by max value
        cubes.for_each(|c| {
            if c.ends_with("red") {
                max_r = std::cmp::max(c.replace("red", "").parse::<u16>().expect(""), max_r);
            }

            if c.ends_with("green") {
                max_g = std::cmp::max(c.replace("green", "").parse::<u16>().expect(""), max_g);
            }

            if c.ends_with("blue") {
                max_b = std::cmp::max(c.replace("blue", "").parse::<u16>().expect(""), max_b);
            }
        });

        // If passes restrictions, add the game id to the total
        if max_r <= 12 && max_g <= 13 && max_b <= 14 {
            total += id;
        }

        // Compute the total power of the game
        total_power += max_r * max_g * max_b;
    }

    println!("Total: {}, Total Power: {}", total, total_power);
    Ok(())
}
