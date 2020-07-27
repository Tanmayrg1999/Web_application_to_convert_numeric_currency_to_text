# Web_application_to_convert_numeric_currency_to_text
## INTRODUCTION TO APPLICATION
Rocket is a web framework for Rust. It is used to design Web applications in the RUST programming languages.Rust is blazingly fast and memory-efficient with no runtime or garbage collector, it can power performance-critical services, run on embedded devices, and easily integrate with other languages. The project can rightly determine the textual output till 100 crore and a decimal precision up-to 2. Program will ask the user to enter a number and the expected output will be the text of corresponding currency. If a number is an integer then it will not return anything after decimal places whereas if the output is a double then it will return the paise corresponding to it divided by 100. The web application supports  5 different languages which includes Hindi,English,Bengali,Gujrati and Marathi and produces the corresponding output
## HOW TO GET STARTED
1. In your system install the nightly version of Rust using the command -> **rustup default nightly**
2. One can use per-directory overrides to use the nightly version only for your Rocket project by running the following command in the directory ->**rustup override set nightly**
3. After installing the nightly version we will create a new binary-based Cargo project using the command -> **cargo new my_web_app --bin**
4. Then we need to move our action to the created directory using the -> **cd my_web_app**
5. Now a cargo.toml file would be created by default we need to add: 
--- 
`[dependencies] `

`rocket = "0.4.5"`

`[dependencies.rocket_contrib]`

`version = "* "`

`default-features = false`

`features = ["json","serve","tera_templates"]`

6. After adding the files to the correct location. Just compile and execute the program my using the command **cargo run**.
7. If the cargo project runs succesfully , then just open the [Link](http://localhost:8000/) in any browser and a form will be displayed on the webpage which asks the user to enter the valid currency and the language in which the user expects the output.
## USER PAGE TO ENTER THE INPUT
![p11](https://user-images.githubusercontent.com/53641559/88573850-03a77c80-d05f-11ea-9f06-b9bc61ef3989.png)
## OUTPUT AS EXPECTED
![p12](https://user-images.githubusercontent.com/53641559/88573852-0609d680-d05f-11ea-996f-96c62ca6c921.png)
## ADDITION OF VALIDATION TO THE WEB APPLICATION
![p13](https://user-images.githubusercontent.com/53641559/88573854-06a26d00-d05f-11ea-8747-7d8eb438fed9.png)
