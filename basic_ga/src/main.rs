use std::env;
use rand::Rng;
fn score(genes: &Vec<u32>) -> Vec<f64>{
    let mut scores: Vec<f64> =  Vec::new();
    for gene in genes.iter() {
        let phenotype = (*gene as f64 - u32::MAX as f64 /2.0 );
        let score = phenotype * phenotype + 100.0 * phenotype.cos();
        scores.push(score);
    }
    scores
}
fn main() {
    //Parameters
    let mut n_generations: i32 = 10;
    let mut  n_samples: i32 = 10;
    let args: Vec<String> = env::args().collect();
    let mut rng = rand::thread_rng();
    //handle args
    for arg in args.iter() {
        let parts: Vec<&str> = arg.split(|c| c == '=').collect();
        if parts.len() < 2 {
            continue;
        }
        let setting = parts[0];
        let value: i32 = parts[1].parse::<i32>().expect("Should"); 
        match setting {
            "--generations" => n_generations = value,
            "--samples" => n_samples = value,
            &_ => println!("Invalid Arg: {}", setting),
        }
    }
    println!("Samples: {}", n_samples);
    println!("Generations: {}", n_generations);
    //Create Gene pool
    let mut pool: Vec<u32> = Vec::new();
    for _i in 0..n_samples {
        pool.push(rng.gen::<u32>());
    }
    //Evolution Process.
    for i in 0..n_generations {
        let scores: Vec<f64> = score(&pool);
    println!("{}", scores[(i % n_samples) as usize]);
    }
}
