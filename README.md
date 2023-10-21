# README [![Rust CI/CD Pipeline](https://github.com/nogibjj/Project8_Vivian/actions/workflows/rustCI.yml/badge.svg)](https://github.com/nogibjj/Project8_Vivian/actions/workflows/rustCI.yml)[![Python CI/CD Pipeline](https://github.com/nogibjj/Project8_Vivian/actions/workflows/pythonCI.yml/badge.svg)](https://github.com/nogibjj/Project8_Vivian/actions/workflows/pythonCI.yml)
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

<img width="940" alt="Screen Shot 2023-10-20 at 11 24 04 PM" src="https://github.com/nogibjj/Project8_Vivian/assets/143654445/47239a60-2a85-494d-bc72-30fa09d808cf">

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

<img width="930" alt="Screen Shot 2023-10-20 at 11 14 22 PM" src="https://github.com/nogibjj/Project8_Vivian/assets/143654445/75c60a6d-78af-4bf5-8b61-fafd2caee67b">


## Compare and Results
Operations Performed
1. Read CSV file
2. Count number of houses
3. Sum median house values

Python Performance Report 
- Time taken: 0.018908977508544922 seconds
- Memory used: 9.37890625 MB

Rust Performance Report
- Time taken: 68.44ms seconds
- Memory used: 0 KB


## Summary
Based on the provided performance reports, when executing the same operations (reading a CSV file, counting the number of houses, and summing the median house values), Python took approximately 0.0189 seconds and consumed about 9.38 MB of memory. On the other hand, the Rust implementation was slightly faster, completing in 68.44 milliseconds, and impressively, it did not increase memory usage, using 0 KB. This indicates that for this specific task, Rust is more efficient in terms of execution time and memory consumption than Python.
