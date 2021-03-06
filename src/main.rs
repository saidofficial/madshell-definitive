use std::io::{self,Write};
use std::str::FromStr;
fn prompt_main() {
    let prChar = "madshell $$>> ";
    print!("{0}", prChar);
    io::stdout().flush().unwrap();
}

fn read_com() -> String {
    let mut userCommand = String::new();
    io::stdin().read_line(&mut userCommand)
        .expect("Failed to read command. Try again");
    println!("!DEBUG: RAW_INPUT: {:?}", userCommand);

    userCommand
}
struct Command {
    keyword : String,
    args : Vec<String>,
}
  
fn tokenizator(c : String) -> Command {
    let mut command_split : Vec<String> = c.split_whitespace().map(|s| s.to_string()).collect();
    println!("!DEBUG: SPLIT_INPUT: {:?}", command_split);
  
    let command = Command {
      keyword : command_split.remove(0),
      args : command_split,
    };
  
    command
}
enum Coms {
    echo,
    ls,
    pwd
}
impl FromStr for Coms {
    type Err = ();
    fn from_string(s: &str) -> Result<Self, Self::Err> {
        match s {
            "echo" => Ok(Coms::echo),
            "ls" => Ok(Coms::ls),
            "pwd" => Ok(Coms::pwd),
            _ => Err(()),
        }
    }
}

fn main() {
loop {

}
}