use std::io::{self,Write};

fn main() {
    let prChar = "root$madshell>>";
    loop {
        print!("{0}", prChar);
        io::stdout().flush().unwrap();

    let mut userCom = String::new();
    io::stdin().read_line(&mut userCom)
        .expect("Failed to read");
    println!("Debug -- {:?}", userCom);

    let comSplit : Vec<&str> = userCom.split(' ').collect();
    println!("Debug -- {:?}", comSplit);

    let keyword = comSplit[0];
    let comArgument = &comSplit[1..];

    println!("Debug -- Keyword: {0}", keyword);
    println!("Debug -- Num of args: {0:?}\nDebug -- Arguments: {1:?}", comArgument.len(), comArgument);
    }
}