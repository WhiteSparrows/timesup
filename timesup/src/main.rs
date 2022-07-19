use std::fs;
use rand::thread_rng;
use rand::seq::SliceRandom;

fn main() {
    let mut team:u8 = 0;
    let mut round:u8 = 1;
    let nb_celebrities: u8 = 10;// maybe choose with an input
    println!("When guessed correctly type a, otherwise type anything else");
    println!("Current team is {team}");
    let filename = "doc/list_celebrities.txt";
    let words:String = import_word(filename);
    let words_vec: Vec<&str> = words.split('\n')
        .into_iter().filter(|x| !x.contains("\t") && !x.is_empty()) 
        .collect();
    let sample: Vec<&str> = words_vec
        .choose_multiple(&mut rand::thread_rng(), nb_celebrities.into())
        .cloned()
        .collect();
    // println!("{:?}", words_vec);
    println!("Current round is {round}");
    let mut words_guessed = Vec::<&str>::new();
    let mut words_wrong_guessed = Vec::<&str>::new();
    let mut rng = thread_rng();
    while words_guessed.len() != nb_celebrities.into(){

        // println!("You have to make guess : {:?}", words_vec.choose(&mut rng));
        let all: Vec<&str> = sample.iter().copied()
            .chain(words_wrong_guessed.iter().copied())
            .collect();
        let to_guess: Vec<_> = all
            .choose_multiple(&mut rand::thread_rng(), 1)
            .collect();
        println!("You have to make guess {:?}", to_guess[0]);
        // insert timer
        let mut line = String::new();
        let b1 = std::io::stdin().read_line(&mut line).unwrap();
        if line.eq("a\n"){
            println!("eq");
            words_guessed.push(to_guess[0]);
            println!("{:?}", words_guessed);
            println!("{:?}", words_guessed.len());
        }
        else{
            println!("not eq");
            //if words_wrong_guessed.iter().any(|&el| el==to_guess[0]){
            if !words_wrong_guessed.contains(to_guess[0]){
                println!("yes");
                words_wrong_guessed.push(to_guess[0]);
            }
            println!("{:?}", words_wrong_guessed);

        }
    }
    
}

fn change_team(team:u8)->u8{
    let new_team: u8 = 1 - team;
    println!("Current team is {new_team}");
    return new_team;
}

fn import_word(filename:&str)->String{
    let content = fs::read_to_string(filename)
        .expect("Something went wrong when reading from the file");
    // println!("The text is {content}");
    return content;
}

