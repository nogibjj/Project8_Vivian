from lib.CRUD import (
    close_connection,
    create_house,
    update_house,
    delete_house,
    read_house_by_id,
    read_all_houses
)
from lib.load_data import load

def count_houses():
    houses = read_all_houses()
    return len(houses)

def main():
    load("california_housing_train.csv")  # Import data from CSV
    number_of_houses = count_houses()
    print(f"There are {number_of_houses} houses in the database.")

    # Close the connection
    close_connection()
    return 1


if __name__ == "__main__":
    main()
