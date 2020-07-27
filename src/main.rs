#![allow(non_snake_case)]
#![allow(unused)]
#![feature(proc_macro_hygiene, decl_macro)]
#[warn(unused_parens)]
#[macro_use]
#[macro_use] extern crate serde_derive;
extern crate serde_json;
use std::ffi::CString;
#[macro_use] extern crate rocket;
use rocket::request::{Form, FormError, FormDataError};
use rocket::response::NamedFile;
use rocket::http::RawStr;
use std::str;
mod english;
mod bengali;
mod gujrati;
mod marathi;
mod hindi;
use std::fs::File;
use std::io::prelude::*;
use rocket::response::Redirect;
use rocket_contrib::templates::Template;
#[cfg(test)] mod tests;
#[derive(Debug, FromFormValue)]
enum FormOption {
    English,Bengali,Gujrati,Marathi,Hindi
}
#[derive(Debug, FromForm)]
struct FormInput<>{
    number: f64,
    Language: FormOption,
}
#[derive(Serialize)]
struct TemplateContext {
    name: String,
    items: String
}
#[post("/",rank=1, data = "<sink>")]
fn sink1(sink:Form<FormInput>)->Template{
let mut output = String::new();
let	x=sink.number;
let intamount= x as i32;
let _diff: f64 = x - f64::from(intamount);
let _fn: f64 = _diff * f64::from(100);
let _int_fn = _fn.round();
let mut name1=String::new();
match sink.Language
{
    FormOption::English=>{
        if intamount< 100 && intamount>0 {
            output.push_str("INR ");
            output.push_str(&*english::hashmap_english2(intamount));
            output.push_str(&(_int_fn).to_string());
            output.push_str("/100");name1.push_str("Your currency value is Rs.");
            name1.push_str(&(sink.number).to_string());
    }
if intamount>100&&intamount<1000000000 {
            output.push_str("INR ");
            output.push_str(&*english::hashmap_english3(intamount));
            output.push_str(&(_int_fn).to_string());
            output.push_str("/100");
            name1.push_str("Your currency value is Rs.");
name1.push_str(&(sink.number).to_string());
        }
}
FormOption::Bengali=>{
    if intamount<1000000000&&intamount>0{
        name1.push_str("তোমার মুদ্রা মান হয় টাকা.");
        name1.push_str(&(sink.number).to_string());
           output.push_str("INR ");
            output.push_str(&*bengali::hashmap_bengali2(intamount));
            output.push_str(&(_int_fn).to_string());
            output.push_str("/100");
    }
}
FormOption::Gujrati=>{
     if intamount<1000000000&&intamount>0{
           name1.push_str("તમારી ચલણ કિંમત રૂ.");
        name1.push_str(&(sink.number).to_string());
         output.push_str("INR ");
            output.push_str(&*gujrati::hashmap_gujrati2(intamount));
            output.push_str(&(_int_fn).to_string());
            output.push_str("/100");}

}
FormOption::Hindi=>{
     if intamount<1000000000&&intamount>0{
         name1.push_str("आपकी मुद्रा का मूल्य रु.");
        name1.push_str(&(sink.number).to_string());
         output.push_str("INR ");
            output.push_str(&*hindi::hashmap_hindi2(intamount));
            output.push_str(&(_int_fn).to_string());
            output.push_str("/100");
     }
}
FormOption::Marathi=>{
         if intamount<1000000000&&intamount>0{
            name1.push_str("तुमचे चलन मूल्य रु.");
            name1.push_str(&(sink.number).to_string());
            output.push_str("INR ");
            output.push_str(&*marathi::hashmap_marathi2(intamount));
            output.push_str(&(_int_fn).to_string());
            output.push_str("/100");
}
}
}
let context = TemplateContext { name:name1, items: output };
 Template::render("index", &context)
}
#[get("/",rank=2)]
fn index() -> Option<NamedFile> {
    NamedFile::open("templates/index.html").ok()
}
fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![sink1,index]).attach(Template::fairing())
}
fn main() {
    rocket().launch();
}
