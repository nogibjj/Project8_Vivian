use std::fs::File;
use csv::Reader;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let number_of_houses = count_houses()?;
    println!("There are {} houses in the dataset.", number_of_houses);

    let total = sum_median_house_values()?;
    println!("Total of all median house values: {}", total);

    Ok(())
}

fn count_houses() -> Result<usize, Box<dyn Error>> {
    let file = File::open("california_housing_train.csv")?;
    let rdr = Reader::from_reader(file);
    Ok(rdr.records().count())
}

fn sum_median_house_values() -> Result<f64, Box<dyn Error>> {
    let file = File::open("california_housing_train.csv")?;
    let mut rdr = Reader::from_reader(file);
    let mut total = 0.0;

    for result in rdr.records() {
        let record = result?;
        let value: f64 = record[8].parse()?;
        total += value;
    }

    Ok(total)
}
