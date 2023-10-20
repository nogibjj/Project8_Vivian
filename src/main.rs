use std::error::Error;
use std::fs::File;

use csv::Reader;

fn main() -> Result<(), Box<dyn Error>> {
    let number_of_houses = count_houses()?;
    println!("There are {} houses in the dataset.", number_of_houses);
    Ok(())
}

fn count_houses() -> Result<usize, Box<dyn Error>> {
    let file = File::open("california_housing_train.csv")?;
    let mut rdr = Reader::from_reader(file);
    let count = rdr.records().count();
    Ok(count)
}
