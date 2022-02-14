use itertools::Itertools;
use std::collections::HashMap;
use std::time::Instant;

static GREEN: u8 = 1;
static YELLOW: u8 = 2;
static GRAY: u8 = 3;

static FIRST_GUESS: &str = "saner";

#[derive(Debug)]
#[derive(Clone)]
pub struct Suggestion {
    pub word: String,
    pub score: f32,
}
 
pub struct Attempt {
    pub word: String,
    pub feedback: [u8; 5],
}

pub fn solve(solution: &str, solutions:Vec<String>, guesses:Vec<String>) -> Vec<Attempt>{
    let mut possibles = solutions.clone();
    let mut answer:String = "".to_string();
    let mut attempts = vec![];
    let mut turn = 0;
    while answer.is_empty(){
        let mut guess = "".to_string();
        if turn == 0 {
            guess = FIRST_GUESS.to_string();
        } else {
            if possibles.len() == 1 {
                guess = possibles[0].to_string();
            } else {
                let suggestions = get_suggestions(possibles.clone(), guesses.clone());
                // for s in suggestions.clone() {
                //     println!("{:?}", s);
                // }
                guess = suggestions[0].clone().word;
            }
        }
        
        let feedback = get_feedback(solution.chars().collect_vec(), guess.chars().collect_vec());
        possibles = trim_possibles(possibles, feedback, guess.to_string());

        // println!("{}", guess.clone());
        
        let attempt = Attempt{
            word: guess.clone(),
            feedback: feedback
        };

        attempts.push(attempt);

        if solution == guess.to_string() {
            answer = guess;
        }
        turn += 1;
    }
    return attempts
}


pub fn get_suggestions(solutions:Vec<String>, guesses:Vec<String>) -> Vec<Suggestion>{
    let now = Instant::now();
    
    let mut chars_possible = vec![];
    let mut chars_all = vec![];
    let all = [solutions.clone(), guesses.clone()].concat();

    for possible in solutions.clone() {
        let solution_vec = possible.chars().collect_vec();
        chars_possible.push(solution_vec);
    }

    for a in all.clone() {
        let a_vec = a.chars().collect_vec();
        chars_all.push(a_vec);
    }
    
    let colors = generate_color_permutations();
    let mut suggestions = vec![];

    for s in 0..chars_possible.len() - 1 {
        let possible = chars_possible.iter().nth(s).unwrap();
        let mut permutations = colors.clone();

        for guess in chars_all.clone() {
            let feedback = get_feedback(possible.clone(), guess);
            permutations.insert(
                feedback.clone(),
                permutations.get_key_value(&feedback).unwrap().1 + 1,
            );
        }
        let mut score = 0.0;
        for permutation in permutations {
            let information = permutation.1 as f32 / all.len() as f32;
            if information != 0.0 {
                let c = (1.0 / information).log2() * information;
                score += c;
            }
        }
        let suggestion = Suggestion {
            word: solutions[s].to_string(),
            score: score,
        };
        suggestions.push(suggestion);
    }
    suggestions.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
    // for s in suggestions.clone() {
    //     println!("{:?}", s);
    // }
    let elapsed = now.elapsed();
    // println!("Elapsed: {:.2?}", elapsed);
    return suggestions;
}

fn generate_color_permutations() -> HashMap<[u8; 5], u16> {
    let combinations = [GREEN, YELLOW, GRAY]
        .into_iter()
        .combinations_with_replacement(5)
        .collect_vec();
    let mut permutations: HashMap<[u8; 5], u16> = HashMap::new();
    for combination in combinations {
        let ps = combination
            .into_iter()
            .permutations(5)
            .unique()
            .collect_vec();
        for permutation in ps {
            let perm = 0;
            let feedback = [
                permutation[0],
                permutation[1],
                permutation[2],
                permutation[3],
                permutation[4],
            ];
            permutations.insert(feedback, perm);
        }
    }
    return permutations;
}

fn get_feedback(solution: Vec<char>, possible: Vec<char>) -> [u8; 5] {
    let mut feedback: [u8; 5] = [0, 0, 0, 0, 0];

    for i in 0..5 {
        let solution_char = solution[i];
        let possible_char = possible[i];

        if solution_char == possible_char {
            feedback[i] = GREEN;
        } else {
            if solution.contains(&possible_char) {
                //TODO: Gray occurence if occurenes in possible > occurences in solution gray
                feedback[i] = YELLOW;
            } else {
                feedback[i] = GRAY;
            }
        }
    }
    return feedback;
}

fn trim_possibles(possibles: Vec<String>, feedback: [u8; 5], solution: String) -> Vec<String> { 
    let mut trimmed: Vec<String> = Vec::new();
    for possible in possibles {
        let mut add = true;
        for i in 0..5 {
            let color = feedback[i];
            let solution_char = solution.chars().nth(i).unwrap();
            let possible_char = possible.chars().nth(i).unwrap();
           
            if color == GREEN {
                if solution_char != possible_char{
                    add = false;
                }
            }

            if color == YELLOW {
                if solution_char == possible_char{
                    add = false;
                }
                if !possible.contains(solution_char) {
                    add = false;
                }
            }

            if color == GRAY {
                if possible.contains(solution_char) { //TODO: gray occurences
                    add = false;
                }
            }
            
        }
        if add == true {
            trimmed.push(possible);
        }
    }
    return trimmed
}