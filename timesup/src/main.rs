use std::fs;

fn main() {
    let mut team:u8 = 0;
    println!("Current team is {team}");
    team = change_team(team);
    println!("Current team is {team}");
    let filename = "doc/list_celebrities.txt";
    let words:String = import_word(filename);
    let v: Vec<&str> = words.split('\n').collect();
    let mut vec = Vec::new();
    let it = v.iter();
    for el in it {
        if !el.contains("\t") && !el.is_empty(){
            vec.push(el);
        }
    }
    println!("{:?}", v);
    println!("{:?}", vec);

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

