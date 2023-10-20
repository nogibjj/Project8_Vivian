import pandas as pd
import time
import psutil

def count_houses(df):
    return len(df)

def sum_median_house_values(df):
    return df['median_house_value'].sum()

def main():
    start_time = time.time()
    memory_before = psutil.virtual_memory().used / (1024.0 ** 2)  # Convert bytes to MB
    
    df = pd.read_csv("california_housing_train.csv")
    number_of_houses = count_houses(df)
    total_median_value = sum_median_house_values(df)
    
    end_time = time.time()
    memory_after = psutil.virtual_memory().used / (1024.0 ** 2)
    
    elapsed_time = end_time - start_time
    memory_used = memory_after - memory_before
    
    with open("python_performance.md", "w", encoding="utf-8") as file:
        file.write("## Python Performance Report\n")
        file.write(f"- Time taken: {elapsed_time} seconds\n")
        file.write(f"- Memory used: {memory_used} MB\n")
        file.write("### Operations Performed\n")
        file.write("1. Read CSV file\n2. Count number of houses\n3. Sum median house values\n")

    print(f"There are {number_of_houses} houses in the dataset.")
    print(f"Total of all median house values: {total_median_value}")

if __name__ == "__main__":
    main()
