import pandas as pd

def count_houses(df):
    return len(df)

def sum_median_house_values(df):
    return df['median_house_value'].sum()

def main():
    df = pd.read_csv("california_housing_train.csv")
    
    number_of_houses = count_houses(df)
    print(f"There are {number_of_houses} houses in the dataset.")
    
    total_median_value = sum_median_house_values(df)
    print(f"Total of all median house values: {total_median_value}")

if __name__ == "__main__":
    main()

