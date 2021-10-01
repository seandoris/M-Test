use crate::{book_change::{reserve_book, return_book}, library_books::read_library};

pub mod book_change;
pub mod library_books;


fn main() {
    let mut exit = false;
    while !exit{
        println!("Welcome to the library! Please select an option. \n
        [V]iew Books,  [R]eturn a Book,  [E]xit Library");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Read user input error");
        let cleaned_input = input.to_string().trim().to_ascii_lowercase().chars().next().unwrap();

        match cleaned_input{
            'v' =>{
                let mut b_list = read_library();
                for i in 0..b_list.len(){
                    println!("Book Number: {} Title: {}, By: {}, Pages: {}\nStatus: {}",
                    i+1, b_list[i].title, b_list[i].author, b_list[i].pages, b_list[i].status);
                    println!("______________________________________");
                }
                println!("Please select an option.\n
                [R]eturn to main menu,  [T]ake out a book");
                input = String::new();
                std::io::stdin().read_line(&mut input).expect("Read user input error");
                let cleaned_input = input.to_string().trim().to_ascii_lowercase().chars().next().unwrap();
                match cleaned_input{
                    'r' => {

                    }
                    't' => {
                        reserve_book(&mut b_list);
                    }
                    _=> {
                        println!("Invalid input {}, please try again!", cleaned_input); 
                    }
                }
            }
            'r' =>{
                return_book(&mut read_library())
            }
            'e'=>{
                exit = true;
            }
            _=>{
                println!("Invalid input {}, please try again!", input);
            }

        }
    }
}
