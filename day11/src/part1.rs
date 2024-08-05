pub fn calculate_distance_between_galaxies(expanded_galaxy: &Vec<Vec<char>>, galaxies: u32) -> u32 {
    let mut galaxy_coordinates: Vec<(usize, usize)> = Vec::with_capacity(galaxies as usize);
    for i in 0..expanded_galaxy.len() {
        for j in 0..expanded_galaxy[0].len() {
            if expanded_galaxy[i][j] == '#' {
                galaxy_coordinates.push((i, j));
            }
        }
    }
    let mut total_distance: u32 = 0;
    for i in 0..galaxy_coordinates.len() {
        for j in i + 1..galaxy_coordinates.len() {
            total_distance += (galaxy_coordinates[i].0 as i32 - galaxy_coordinates[j].0 as i32)
                .abs() as u32
                + (galaxy_coordinates[i].1 as i32 - galaxy_coordinates[j].1 as i32).abs() as u32;
        }
    }
    total_distance
}
