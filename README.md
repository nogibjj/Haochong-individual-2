# Haochong-individual-project-2 


This is a repo template for course 706_Data_Engineering Individual Project 2. First of all, I create this Rust project with a `Cargo.toml` inside and I use the same dataset in week 2 and 5. Secondly, I define functions called `extract` to get data from url. Then, use `load_transform` to create database. After that, I use `connect` to connect database.  Next, I create the table "indexs" and perform CRUD by `create_table`, `read`, `update_shape_leng` and `delete`. I also define a function called `insert` to help me with CRUD. Consequently, I use `main.rs` to use my function in `lib.rs`, and use the output of `main.rs` and `test_main.rs` to test my functions. Finally, I use Action to run `Makefile` and got a 100% pass. 

Important file:
* `Makefile`
* `RustCI.yml`
* `lib.rs`
* `main.rs`
* `test_main.rs`
* `25ktopomapseriesindex.csv`
* `ktopomapseriesindex.db`
* `Cargo.toml`

## Youtube Video Link 


## Purpose
- Rust source code: The code should comprehensively understand Rust's syntax and unique features.
- Use of GitHub Copilot: In your README, explain how you utilized GitHub Copilot in your coding process.
- SQLite Database: Include a SQLite database and demonstrate CRUD (Create, Read, Update, Delete) operations.
- Optimized Rust Binary: Include a process that generates an optimized Rust binary as a GitHub Actions artifact that can be downloaded.
- README.md: A file that clearly explains what the project does, its dependencies, how to run the program, and how GitHub Copilot was used.
- GitHub Actions: A workflow file that tests, builds, and lints your Rust code.
- Video Demo: A YouTube link in README.md showing a clear, concise walkthrough and demonstration of your CLI binary.


### Preparation: 
1. open codespaces 
2. wait for codespaces to be built 
3. build: `cargo build`
4. run: `cargo run`
5. test: `cargo test`

### Using Copilot:
- I use copilot in two ways while writing my code:
1. Press `Cmd+I` on macOS (or `Ctrl+I` on Windows/Linux) to bring up the Chat input to send request to Copilot. After that, I can choose if I acceptthe suggestion from copilot to write my code. In addition, the suggestion will not be fully correct. I need to modify the code to make it correct, but it provide me a template to write my code, which helps me to write code faster with less errors.

![Alt text](<Copilot 1.png>)

2. Copilot will also suggest me some contents automatically while I am writing my code. I can press `Tab` to accept the suggestion from copilot, which also helps me to write code faster with with less errors.

![Alt text](<Copilot 2.png>)

- Hence, there are two advantages of using copilot:
1. Copilot helps me to write code faster. 
2. Copilot helps me to write code with less errors. 


### Components:

1. **Extraction:**
    - The `extract` function copy contents of a csv from a specified URL and saves it to a local file by relative path.

2. **Create Table("C" in "CRUD"):**
    - The `create_table` function create a table called "indexs".

3. **Loading and Transformation:**
    - The `load_transform` function reads the CSV dataset from the file by relative path and inserts its records into a SQLite database.

4. **Insert Data:**
    - The `insert` function inserts a new record into the database. This is a function to help me with CRUD.

5. **Read Data("R" in "CRUD"):**
    - The `read` function reads the data from the database and returns a vector of `Index` structs.

6. **Update Data("U" in "CRUD"):**
    - The `update_shape_leng` function updates the `Shape_Leng` field of a record in the database.

7. **Delete Data("D" in "CRUD"):**
    - The `delete` function deletes a record from the database.



### Check Format and Test Erros: 
1. Format code `make format`
2. Lint code `make lint`
3. Test coce `make test`

- I write test code in `test_main.rs` and use `cargo test` to test my code. There are three tests in `test_main.rs`:
1. `test_extract_and_load`: test `extract` and `load_transform` function.
2. `test_update_shape_leng`: test `update_shape_leng` function by checking the `Shape_Leng` field of a record in the database using "assert_eq!".
3. `test_insert_and_delete()`: test `insert` function by checking all 4 fields of a record just inserted in the database using "assert_eq!". Then, test `delete` function by checking the record just deleted in the database using "assert_ne!".

- I didn't write test for `create_table` and `read` function because I think it is not necessary. I can see the result of `create_table` and `read` function by using `cargo run`. There will be no result if there is an error for `create_table` and `read` function.

![Alt text](<print result.png>)


![Alt text](<test result.png>)


### Optimized Rust Binary
- Method: Add a new step to the workflow file that generates an optimized Rust binary as a GitHub Actions artifact that can be downloaded.

- Download the artifact: Click `actions` and then click the latest `RustCI` workflow. You will be able to find and download the artifact here.

![Alt text](<截屏2023-10-24 下午11.53.06.png>)


## References
* https://github.com/nogibjj/rust-data-engineering
* https://code.visualstudio.com/blogs/2023/03/30/vscode-copilot
