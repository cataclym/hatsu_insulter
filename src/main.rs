use std::io::{self, BufRead};
use rand::{self, Rng};

fn main() {
    let verbs = ["sucks", "sniffs", "licks", "eats", "rubs on", "is aroused by", "squeezes", "fiddles with", "kisses", "wiggles on"];
    let nouns = ["dick", "ass juice", "feet", "armpits", "poop", "vomit", "earwax", "belly button dirt", "man boobs", "chest hair", "pubes", "toe nails", "pimples", "facc", "otaku sweat"];
    println!("Press any key to insult Hatsu. Enter 0 to exit");

    loop {
        let input = io::stdin().lock().lines().next().unwrap().unwrap();
        if input == "0" {
            break;
        }
        let _rng = rand::thread_rng();
        let verb = verbs[rand::thread_rng().gen_range(0..verbs.len())];
        let noun = nouns[rand::thread_rng().gen_range(0..nouns.len())];
        println!("{}", format!("Hatsu {} {}", verb, noun));
    }
}