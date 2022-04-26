use rand::Rng;

const DNA_COUNT: usize = 30;

fn random_string() -> String {
    let mut rng = rand::thread_rng();
    let start = 'a';
    let end = 'z';
    let strings: Vec<char> = (start..end).collect();
    strings[rng.gen_range(0..(start..end).count())].to_string()
}

fn calc_score(target: &str, dna: &str) -> usize {
    target.chars().enumerate().fold(0, |acc, (i, c)| {
        acc + (if c == dna.chars().nth(i).unwrap() {
            1
        } else {
            0
        })
    })
}

fn init_generation(target: &str) -> Vec<String> {
    (0..DNA_COUNT)
        .map(|_i| {
            target
                .chars()
                .map(|_c| random_string())
                .collect::<Vec<String>>()
                .join("")
        })
        .collect()
}

fn next_dna(pool: &[String]) -> String {
    let mut rng = rand::thread_rng();
    let parent1 = pool.get(rng.gen_range(0..pool.len())).unwrap().to_string();
    let parent2 = pool.get(rng.gen_range(0..pool.len())).unwrap().to_string();

    let mid = rng.gen_range(0..parent1.len());
    (0..parent1.len())
        .map(|i| {
            if rng.gen_range(0..100) == 0 {
                return random_string();
            }
            (if i < mid { &parent1 } else { &parent2 })
                .chars()
                .nth(i)
                .unwrap()
                .to_string()
        })
        .collect::<Vec<String>>()
        .join("")
}

pub fn run() {
    let target = "helloworld";

    let mut generation = init_generation(target);

    for i in 0..10000 {
        let scores: Vec<usize> = generation
            .iter()
            .map(|dna| calc_score(target, dna))
            .collect();
        let max = scores.iter().max().unwrap();
        let average = scores.iter().sum::<usize>() as f64 / scores.len() as f64;
        let max_dna = generation
            .iter()
            .find(|dna| calc_score(target, dna) == *max)
            .unwrap();

        let mut pool = Vec::<String>::new();
        for dna in &generation {
            let score = calc_score(target, dna);
            for _ in 0..(score + 1) {
                pool.push(dna.to_string());
            }
        }

        println!("{} | {:.2} | {} | {}", i, average, max, max_dna);

        if target == max_dna {
            break;
        }

        generation = (0..generation.len())
            .map(|_| next_dna(&pool))
            .collect::<Vec<String>>();
    }
}
