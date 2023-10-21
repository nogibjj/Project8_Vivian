// test_main.rs

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_houses() {
        let data = "
            longitude,latitude,housing_median_age,total_rooms,total_bedrooms,population,households,median_income,median_house_value
            -122.23,37.88,41.0,880.0,129.0,322.0,126.0,8.3252,452600.0
            -122.22,37.86,21.0,7099.0,1106.0,2401.0,1138.0,8.3014,358500.0
        ";
        let file = File::from(data);
        let count = count_houses(file).unwrap();
        assert_eq!(count, 2);
    }

    #[test]
    fn test_sum_median_house_values() {
        let data = "
            longitude,latitude,housing_median_age,total_rooms,total_bedrooms,population,households,median_income,median_house_value
            -122.23,37.88,41.0,880.0,129.0,322.0,126.0,8.3252,452600.0
            -122.22,37.86,21.0,7099.0,1106.0,2401.0,1138.0,8.3014,358500.0
        ";
        let file = File::from(data);
        let sum = sum_median_house_values(file).unwrap();
        assert_eq!(sum, 452600.0 + 358500.0);
    }
}
