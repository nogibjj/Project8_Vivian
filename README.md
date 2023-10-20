# README [![CI](https://github.com/nogibjj/Project8_Vivian/actions/workflows/ci.yml/badge.svg)](https://github.com/nogibjj/Project8_Vivian/actions/workflows/ci.yml)
This repository features the materials for Mini-Project 8. It includes: 
- A Makefile
- A Dockerfile
- A foundational set of libraries for development operations and web applications
- GitHub Actions
- Cargo.toml to handle Rust dependencies
- Rust and Python scripts


# Purpose Of Project
The purpose of this project is to capture the performance and resource consumption between python and Rust when dealing with data processing works. I use the same dataset from my previous project "california_housing.csv", read the CSV file and calculated the total number of houses and summed up their median values.

# Steps
1. open the project in codespaces
2. container built and virtual environment to be activated through requirements.txt
3. Compile and Run Rust: cargo run
4. 

# Check Format & Errors
1. make format
2. make lint
4. make test

<img width="940" alt="Screen Shot 2023-09-30 at 11 43 41 AM" src="https://github.com/nogibjj/Project5_Vivian/assets/143654445/bd7caa1e-cc41-4645-8af3-6929eef41df2">

<img width="939" alt="Screen Shot 2023-09-30 at 11 44 04 AM" src="https://github.com/nogibjj/Project5_Vivian/assets/143654445/aa1f500f-2b0a-49b9-8dc3-27022c279d79">

# What I did:
For the Rust part:
- wrote a Rust program to read a CSV file containing housing details and calculated the total number of houses and summed up their median values.
- Integrated functionalities to capture its execution time and memory usage to assess the performance of my Rust implementation.
- The performance data is documented in a markdown file named "rust_performance.md", showcasing the efficiency and resource management capabilities of Rust in handling the dataset.

For the Python part:
- crafted a Python script that reads the same housing dataset, counts the number of houses, and calculates the total of the median house values.
- captured the script's execution time and memory consumption and recorded them in a markdown file "python_performance.md".






# Compare and Results


Below shows the output after each CRUD operations on my dataset: 

<img width="766" alt="Screen Shot 2023-09-30 at 11 44 28 AM" src="https://github.com/nogibjj/Project5_Vivian/assets/143654445/c0f2da64-ff73-40fc-972c-890b71a06fd1">
