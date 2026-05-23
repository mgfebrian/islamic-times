pub fn get_compass_direction(degree_direction: &f64) -> &'static str {
    let normalized = (degree_direction % 360.0) + 360.0;
    let index = ((normalized + 22.5) / 45.0).floor() as usize % 8;

    let direction = [
        "N",
        "NE",
        "E",
        "SE",
        "S",
        "SW",
        "W",
        "NW",
    ];

    direction[index]
}