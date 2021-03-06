# Web_application_to_convert_numeric_currency_to_text
## INTRODUCTION TO APPLICATION
Rocket is a web framework for Rust. It is used to design Web applications in the RUST programming languages.Rust is blazingly fast and memory-efficient with no runtime or garbage collector, it can power performance-critical services, run on embedded devices, and easily integrate with other languages. The project can rightly determine the textual output till 100 crore and a decimal precision up-to 2. Program will ask the user to enter a number and the expected output will be the text of corresponding currency. If a number is an integer then it will not return anything after decimal places whereas if the output is a double then it will return the paise corresponding to it divided by 100. The web application supports  5 different languages which includes Hindi,English,Bengali,Gujrati and Marathi and produces the corresponding output also a functionalty to copy the resulting output was added by providing a copy button.
## HOW TO GET STARTED
1. In your system install the nightly version of Rust using the command -> **rustup default nightly**
2. One can use per-directory overrides to use the nightly version only for your Rocket project by running the following command in the directory ->**rustup override set nightly**
3. After installing the nightly version we will create a new binary-based Cargo project using the command -> **cargo new my_web_app --bin**
4. Then we need to move our action to the created directory using the -> **cd my_web_app**
5. Now a cargo.toml file would be created by default we need to add: 
--- 
`[dependencies] `

`rocket = "0.4.5"`

`htmlescape = "0.3.1"`

`serde = "1.0"`

`serde_derive = "1.0"`

`serde_json = "1.0"`

`[dependencies.rocket_contrib]`

`version = "* "`

`default-features = false`

`features = ["json","serve","tera_templates"]`

6.Create a template folder so as to store the index.html and index.html.tera file in the new package created.

7. After adding the files to the correct location. Just compile and execute the program my using the command **cargo run**.

8. If the cargo project runs succesfully , then just open the [Link](http://localhost:8000/) in any browser and a form will be displayed on the webpage which asks the user to enter the valid currency and the language in which the user expects the output.
## WHAT IS INDEX.HTML
This is the page where our initial form is available or the page where the user is made to enter the details. This is a basic html form with two input types namely the number and the language in which the user expects the output. Also some form validations are added as like the user has to enter the number else it generates a warning. another validation is that if the user enters the empty number then a validation runs which requests user to enter a number. Also the number must contain maximum of 9 digits before the decimal point and also it prohibits user to enter a negative value. One more validation is that the user must include maximum of 2 numbers after the decimal points. The user has to click the check button and the page gives the index.html.tera as the output. 
## WHAT IS INDEX.TERA.HTML
A Tera template is just a text file where variables and expressions get replaced with values when it is rendered. the format for this tera file is similar to that of html file. The first entity that was added to the tera file was the currency textual output. This was done by the use of simple description list using the **"dl** tags in HTML and "items" was the textual output written inside **"{{items}}"**. Another functionality was added to the tera file which copied the textual output on a click for this a small css code was used which copied the data using the function named "myfunction()" and on succesfull copying it poped up a text showing "text copied !".All you need is to just click on the "Click me to copy to clipboard".

## LEARNING OUTCOMES FROM PROJECT
1.Designing of flowchart to understand the program flow approach.<br>
2.Importance of Source Control and why is it necessary to locally store your program status.<br>
3.Naming variables in a user understandable fashion so to make program more user friendly.<br>
4.Comments in code make the code more readable and understandable.<br>
5.Importance of unit tests.<br>
6.What is a code coverage and how to improve the code coverage.<br>
7.How to shift from one language to another keeping the semantic logic same.<br>
8.Basic syntax of RUST and advantages of RUST over other languages.<br>
9.Error handling and adding constraints to WEB API.<br>
10.File handling in RUST and C++.<br>
11.Designing a webpage or a page by the functionalities of HTML ,CSS and JavaScript.<br>
12.Working with UI Frameworks. I searched a lot about the web UI and found that Actix web and Rocket are useful for designing WEB API in RUST Language and came up with choosing Rocket framework.<br>
## USER PAGE TO ENTER THE INPUT
![p11](https://user-images.githubusercontent.com/53641559/88573850-03a77c80-d05f-11ea-9f06-b9bc61ef3989.png)
## OUTPUT AS EXPECTED
![p12](https://user-images.githubusercontent.com/53641559/88573852-0609d680-d05f-11ea-996f-96c62ca6c921.png)
## ADDITION OF VALIDATION TO THE WEB APPLICATION
![p13](https://user-images.githubusercontent.com/53641559/88573854-06a26d00-d05f-11ea-8747-7d8eb438fed9.png)
