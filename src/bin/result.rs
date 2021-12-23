#[derive(Debug)]
enum MenuChoice {
    MainMenu,
    Start,
    Quit,
}

fn get_choice(input: &str) -> Result<MenuChoice, String> {
    match input {
        "mainmenu" => Ok(MenuChoice::MainMenu),
        "start" => Ok(MenuChoice::Start),
        "quit" => Ok(MenuChoice::Quit),
        _ => Err("menu choice not found".to_owned()),
    }
}

fn print_choice(choice: &MenuChoice) {
    println!("choice: {:?}", choice)
}

fn pick_choice(input: &str) -> Result<(), String> {
    //()  represents nothing.
    let choice = get_choice(input)?; //Note: ? here
    print_choice(&choice);
    Ok(()) // returning nothing
}
fn main() {
    let choice = pick_choice("leave");
    println!("choice: {:?}", choice);
}
