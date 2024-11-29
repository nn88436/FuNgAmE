use rand::Rng;
use std::collections::HashMap;
fn main() {
    let possiblilities = HashMap::from([(1, "Rock"), (2, "Paper"), (3, "Scissor")]);
    generate_random(possiblilities)
}

fn generate_random(possiblilities: HashMap<i32, &str>) {
    let first_person = find_match(possiblilities.get(&rand::thread_rng().gen_range(1..4)));
    let second_person = find_match(possiblilities.get(&rand::thread_rng().gen_range(1..4)));

    game_rules(&first_person, &second_person);
}

fn find_match(some_value: Option<&&str>) -> String {
    let out = match some_value {
        None => "",
        Some(i) => i,
    };
    return out.to_string();
}

fn game_rules(first_p_guess: &str, second_p_guess: &str) {
    let result = match first_p_guess {
        "Rock" => match second_p_guess {
            "Rock" => String::from("Draw"),
            "Paper" => String::from("Win second person"),
            _ => String::from("Win first person"),
        },
        "Paper" => match second_p_guess {
            "Paper" => String::from("Draw"),
            "Scissor" => String::from("Win second person"),
            _ => String::from("Win first person"),
        },
        "Scissor" => match second_p_guess {
            "Scissor" => String::from("Draw"),
            "Rock" => String::from("Win second person"),
            _ => String::from("Win first person"),
        },
        _ => String::from("Invalid option"),
    };

    print!(
        "First person : {:?}  Second person : {:?} \n",
        first_p_guess, second_p_guess,
    );
    print!(" Game Results :  {:?} \n", result);
}
