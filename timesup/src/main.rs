use std::fs;

fn main() {
    let mut team:u8 = 0;
    println!("Current team is {team}");
    team = change_team(team);
    println!("Current team is {team}");
    let filename = "doc/list_celebrities.txt";
    let words:String = import_word(filename);
    let words_vec: Vec<&str> = words.split('\n')
        .into_iter().filter(|x| !x.contains("\t") && !x.is_empty()) 
        .collect();
    println!("{:?}", words_vec);

}

fn change_team(team:u8)->u8{
    return 1 - team
}

fn import_word(filename:&str)->String{
    let content = fs::read_to_string(filename)
        .expect("Something went wrong when reading from the file");
    // println!("The text is {content}");
    return content;
}

