use std::env;
use rand::Rng;
//Sample struct contains samples gene and score
struct Sample {
    gene: u32,
    score: f64
}
fn score(gene: u32) -> f64 {
        let phenotype = gene as f64 - u32::MAX as f64 /2.0 ;
        phenotype * phenotype + 100.0 * phenotype.cos()
}
//This is possibly rendundant
fn score_vec(genes: &Vec<u32>) -> Vec<f64>{
    let mut scores: Vec<f64> =  Vec::new();
    for gene in genes.iter() {
        scores.push(score(*gene));
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
    let mut pool: Vec<Sample> = Vec::new();
    for _i in 0..n_samples {
        let gene:u32 = rng.gen::<u32>();
        pool.push(Sample {
            gene: gene,
            score: score(gene),
        });
    }
    //Evolution Process.
    for i in 0..n_generations {
        //sort samples by score
        pool.sort_by(|s_1, s_2| s_1.score.partial_cmp(&s_2.score).unwrap());
        println!("Best is: {}", pool[0].score);
        println!("Worst is: {}", pool[(n_samples - 1) as usize].score);
    }
}
