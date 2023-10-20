# README [![CI](https://github.com/nogibjj/Project8_Vivian/actions/workflows/ci.yml/badge.svg)](https://github.com/nogibjj/Project8_Vivian/actions/workflows/ci.yml)
This repository features the materials for Mini-Project 8. It includes: 
- A Makefile
- A Dockerfile
- A foundational set of libraries for development operations and web applications
- GitHub Actions
- Cargo.toml to handle Rust dependencies
- Rust and Python scripts


## Purpose Of Project
The purpose of this project is to capture the performance and resource consumption between python and Rust when dealing with data processing works. I use the same dataset from my previous project "california_housing.csv", read the CSV file and calculated the total number of houses and summed up their median values.



## Rust Part
- wrote a Rust program to read a CSV file containing housing details and calculated the total number of houses and summed up their median values.
- Integrated functionalities to capture its execution time and memory usage to assess the performance of my Rust implementation.
- The performance data is documented in a markdown file named "rust_performance.md", showcasing the efficiency and resource management capabilities of Rust in handling the dataset.

### Steps
1. open the project in codespaces
2. container built and virtual environment to be activated through requirements.txt
3. build Rust: cargo build
4. run Rust: cargo run
   
### Check Format & Errors
- make format
- make lint
- make test

# Python Part
For the Python part:
- crafted a Python script that reads the same housing dataset, counts the number of houses, and calculates the total of the median house values.
- captured the script's execution time and memory consumption and recorded them in a markdown file "python_performance.md".

### Steps
1. install: make python_install
2. run: python main.py

### Check Format & Errors
- make python_format
- make python_lint
- make python_test


## Compare and Results


