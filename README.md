# Individual Project 2

[![Clippy](https://github.com/nogibjj/individual_project_2/actions/workflows/lint.yml/badge.svg)](https://github.com/nogibjj/individual_project_2/actions/workflows/lint.yml)
[![Tests](https://github.com/nogibjj/individual_project_2/actions/workflows/tests.yml/badge.svg)](https://github.com/nogibjj/individual_project_2/actions/workflows/tests.yml)
[![Rustfmt](https://github.com/nogibjj/individual_project_2/actions/workflows/rustfmt.yml/badge.svg)](https://github.com/nogibjj/individual_project_2/actions/workflows/rustfmt.yml)
[![Build binary release](https://github.com/nogibjj/individual_project_2/actions/workflows/release.yml/badge.svg)](https://github.com/nogibjj/individual_project_2/actions/workflows/release.yml)

## Goals 

The goal of this project was to create a Rust repository that is able to create an SQLite databse, populate it with any csv document. I used a `diabetes.csv` found online and created a `Diabetes.db` for which i performed CRUD-operations. This project was created while adhering to DevOps principles, and is fully tested, linted, formatted, and built utilising Git-Hub Co-Pilot. A deeper explanation of the project, and how to run it, is explained below.

## Github Co-Pilot

I utilised the functionality of github co-pilot and the ability to transform code into different packages. The generative AI tool was able to help produce code in rust syntax with ease, Co-Pilot was also able to guide my syntax to produce code which was reproducible in Rust.

## C.R.U.D. Operations

The following below are screenshots that demonstrates that the code supports C.R.U.D operations (Create, Read, Update, Delete). All of them are SQL queries that were successfully executed. As well, I used Copilot to explain the different parts of functions that rusqlite required I used for query execution, as there wasn't a great explanation in the crate's documentation.

## Create & Read

The code is able to create and read a table from the csv and create a data base for which queries can be run. 
From the image above we Select all columns from the database and produce a limit of 5 queries on the command line.

<img width="1069" alt="Screenshot 2023-10-27 at 4 41 56 PM" src="https://github.com/nogibjj/individual_project_2/assets/125210401/4d1c3633-7261-4642-a19c-e18d8ad8a753">

## Delete

The Outcome variable is `1` and `0` the query deletes the values with `0` which is an indication the patient doesnt have diabetes.
Below is the query ran as a command line tool.

<img width="1082" alt="Screenshot 2023-10-27 at 4 45 24 PM" src="https://github.com/nogibjj/individual_project_2/assets/125210401/eea5449d-8702-4187-91aa-baf6e9bd6eb4">

# Update

The final query updates the Glucose column where it is `140` to `3000` and prints out the databse using the command line tool as seen below.

<img width="1059" alt="Screenshot 2023-10-27 at 4 49 12 PM" src="https://github.com/nogibjj/individual_project_2/assets/125210401/31b90d74-9a6c-4dbc-9f92-f9cc65573c47">


<img width="1082" alt="Screenshot 2023-10-27 at 4 53 36 PM" src="https://github.com/nogibjj/individual_project_2/assets/125210401/21e3da6b-33b5-4181-9053-4634c92bc82e">

## Demo Video


