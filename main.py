import pandas as pd

def read_all_houses():
    return pd.read_csv("california_housing_train.csv")

def count_houses():
    houses = read_all_houses()
    return len(houses)

def main():
    number_of_houses = count_houses()
    print(f"There are {number_of_houses} houses in the dataset.")
    return 1

if __name__ == "__main__":
    main()

