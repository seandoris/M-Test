
use crate::library_books::{Book, Library};
use std::io::prelude::*;
use std::fs::File;


/* Opens the provided book list and provides the user to reserve a book by
the provided book number given by the UI (equivalent to books index + 1). 
Blocks users from taking out books that are already out, or 
book numbers that don't exist within the system */
pub fn reserve_book(b_list: &mut Vec<Book>){
    println!("\nPlease select a book number you would like to take out. Or [E]xit.");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Read user input error");
    let trimmed = input.trim();
    match trimmed.parse::<u32>(){
        Ok(i)=>{
            let book_num = (i-1) as usize;
            if book_num> b_list.len(){
                println!("Invalid book number please try again");
                reserve_book(b_list);
            }
            else{
                if b_list[book_num].status=="Available"{
                    b_list[book_num].status = "Out".to_string();
                    println!("Thank you for reserving {}, hope you enjoy your reading", b_list[book_num].title);
                    save_library(b_list.clone());
                }
                else{
                    println!("Sorry, that book is already out.");
                    reserve_book(b_list)
                }
                
            }
        }
        Err(..)=>{
            if trimmed.to_ascii_lowercase().chars().next().unwrap()!='e'{
                println!("Input is not valid, please try again.");
                reserve_book(b_list);
            }
        }
    }
}


/* Provides user interface for returning books, only allows users to return books
that are actively out. The UI displays all books that are currently listed as being
out of the library. */
pub fn return_book(b_list: &mut Vec<Book>){
    let mut input = String::new();
    println!("\nPlease select a book number you would like to return. Or press [E]xit.");
    println!("______________________________________");
    for i in 0..b_list.len(){
        if b_list[i].status == "Out"{ 
            println!("Book Number: {} Title: {}, By: {}, Pages: {}\nStatus: {}",
            i+1, b_list[i].title, b_list[i].author, b_list[i].pages, b_list[i].status);
            println!("______________________________________");
        }
    }
    std::io::stdin().read_line(&mut input).expect("Read user input error");
    let trimmed = input.trim();
    match trimmed.parse::<u32>(){
        Ok(i)=>{
            let book_num = (i-1) as usize;
            if book_num> b_list.len(){
                println!("Invalid book number please try again");
                reserve_book(b_list);
            }
            else{
                if b_list[book_num].status=="Out"{
                    b_list[book_num].status = "Available".to_string();
                    println!("Thank you for returning {}.", b_list[book_num].title);
                    save_library(b_list.clone());
                }
                else{
                    println!("Sorry, that book is already has already been returned.\n
                    Please try again or contact the librarian if you believe this is a mistake.");
                }
            }
        }
        Err(..)=>{
            if trimmed.to_ascii_lowercase().chars().next().unwrap()!='e'{
                println!("Input is not valid, please try again.");
                reserve_book(b_list);
            }
        }
    }
}

/* Function for saving the changes to book statuses to the library.json file. */
fn save_library(b_list: Vec<Book>){
    let lib = Library{
        books:b_list,
    };
    let test = serde_json::to_string(&lib).unwrap();
    let json_format = serde_json::to_string_pretty(&lib).expect("Unable to convert struct to string");
    println!("{}\n{}",test,json_format);
    let mut f = File::create("library.json").expect("unable to open library to write");
    f.write_all(json_format.as_bytes()).expect("Unable to write to library");

}