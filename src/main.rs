use std::fs::File;
use csv::Reader;
use std::error::Error;
use std::time::Instant;
use sys_info;

fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();
    let mem_info_before = sys_info::mem_info().unwrap();

    let number_of_houses = count_houses()?;
    println!("There are {} houses in the dataset.", number_of_houses);

    let total = sum_median_house_values()?;
    println!("Total of all median house values: {}", total);
    
    let elapsed = start.elapsed();
    let mem_info_after = sys_info::mem_info().unwrap();
    let mem_used = mem_info_after.total - mem_info_before.total;

    std::fs::write(
        "rust_performance.md",
        format!(
            "## Rust Performance Report\n- Time taken: {:.2?} seconds\n- Memory used: {} KB\n### Operations Performed\n1. Read CSV file\n2. Count number of houses\n3. Sum median house values\n", 
            elapsed, mem_used
        )
    )?;

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
