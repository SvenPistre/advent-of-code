use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    first_puzzle()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_location_id_lists_from_file<P>(filename: P) -> Option<(Vec<u32>, Vec<u32>)>
where
    P: AsRef<Path>,
{
    if let Ok(lines) = read_lines(filename) {
        let mut location_ids_group1: Vec<u32> = vec![];
        let mut location_ids_group2: Vec<u32> = vec![];
        for line in lines.flatten() {
            let split_line = line.trim().split_whitespace().collect::<Vec<&str>>();
            let location_id1 = split_line[0].parse::<u32>();
            if let Ok(location_id) = location_id1 {
                location_ids_group1.push(location_id)
            }
            let location_id2 = split_line[1].parse::<u32>();
            if let Ok(location_id) = location_id2 {
                location_ids_group2.push(location_id)
            }
        }
        return Some((location_ids_group1, location_ids_group2));
    }
    None
}

fn first_puzzle() -> () {
    if let Some((mut location_id_group1, mut location_id_group2)) =
        get_location_id_lists_from_file(Path::new("./day-1/inputs/input.txt"))
    {
        location_id_group1.sort();
        location_id_group2.sort();
        let distance = location_id_group1
            .iter()
            .zip(location_id_group2.iter())
            .fold(0, |prev, (id1, id2)| prev + id1.abs_diff(*id2));
        println!("The total distance between both lists is {}", distance)
    }
}
