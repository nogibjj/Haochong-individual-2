# Haochong-week8-mini-repo 
[![PYTHONCI](https://github.com/nogibjj/Haochong-Week-8-mini/actions/workflows/pytest.yml/badge.svg)](https://github.com/nogibjj/Haochong-Week-8-mini/actions/workflows/pytest.yml)
[![Clippy](https://github.com/nogibjj/Haochong-Week-8-mini/actions/workflows/lint.yml/badge.svg)](https://github.com/nogibjj/Haochong-Week-8-mini/actions/workflows/lint.yml)
[![Tests](https://github.com/nogibjj/Haochong-Week-8-mini/actions/workflows/tests.yml/badge.svg)](https://github.com/nogibjj/Haochong-Week-8-mini/actions/workflows/tests.yml)

This is a repo template for course 706_Data_Engineering Week 8 Mini Project. First of all, I write my code in python to read a csv and get the median value of to numeric columns. Then, create a `Cargo.toml`and transform my code into rust and create `lib.rs` and `main.rs` . After that, I create `test_main.rs` to test the functions. Finally, I add content for both rust and python in `Makefile`, and use Action to run `Makefile` and got a 100% pass. 

Important file:
* `Makefile`
* `lib.py`
* `main.py`
* `test_main.py`
* `lib.rs`
* `main.rs`
* `test_main.rs`
* `25ktopomapseriesindex.csv`
* `Cargo.toml`

## Purpose
- Take an existing Python script for data processing
- Rewrite it in Rust
- Highlight improvements in speed and resource usage


## Rust Implementation:
The Rust version reads data from my CSV file in main function by using external crates `csv`, then calculates the medians of two numeric columns of `shape_leng` and `shape_area` using my function `calculate_median`, which use a reference to a vector of f64 (64-bit floating-point numbers) as its argument, then create a mutable copy of the argument and sort it in ascending order using the `sort_by method`. In the sorting process, in order to deal with potential NaN values, I also use `a.partial_cmp(b).unwrap()`. Then, the function gets the length of the sorted vector and return the middle value if length is odd, or return the average of the two middle values if length is even. Also, as required, I use `Instant` to collect start time and end time and then calculate the usage of time. In addition, I use `sys_info::mem_info` to get memmory usage and `std::process::Command` to get cpu usage for my rust code.


### Preparation: 
1. open codespaces 
2. wait for codespaces to be built 
3. build: `cargo build`
4. run: `cargo run`

### Check Format and Test Erros: 
1. Format code `make format`
2. Lint code `make lint`
3. Test coce `make test`

![Alt text](<截屏2023-10-20 下午5.51.24.png>)

## Python Implementation:
For python, I just use `panda` to read csv and use `median` to get medians of `shape_leng` and `shape_area`. Besides, I import package `psutil` to get cpu usage by `psutil.cpu_percent()` and memory usage by `psutil.virtual_memory()`.


### Preparation: 
1. open codespaces 
2. wait for codespaces to be built 
3. run `main.py`  

### Check Format and Test Erros: 
1. Format code `make python_format`
2. Lint code `make python_lint`
3. Test coce `make python_test`

![Alt text](<截屏2023-10-20 下午9.39.16.png>)

## Speed and Resource Usage:
1. Python:

![Alt text](<截屏2023-10-20 下午9.22.40.png>)

* time: 0.0043s
* CPU Usage: 75.0%
* Memory Usage: 80.3%

2. Rust: 

![Alt text](<截屏2023-10-20 下午9.19.54.png>)

* time: 0.002593167s
* CPU Usage: 10.20%
* Memory Usage: 81.26021%

The results of the medians are both correct. We can see that the cpu usage and running time of Rust are way more lower than Python. Rust is a statically compiled language, which means its code is compiled into machine code before execution, while Python is an interpreted language, which means it is more flexible since the code is executed by the Python interpreter at runtime and requires more works to run. Besides, Rust is also a statically typed language, which means that type checking is finished at compile-time, hence many errors can be catched before runtime. However, Python is dynamically typed, which means type checking occurs at runtime. Therefore, Python also becomes more flexible in this way but it also can lead to more runtime checks that consume more time.


## References
* https://github.com/nogibjj/rust-data-engineering
