use std::env;
use rand::Rng;
use rand::rngs::ThreadRng;

//Sample struct contains samples gene and score
struct Sample {
    gene: u32,
    score: f64
}
fn score(gene: u32) -> f64 {
        let phenotype = gene as f64 - u32::MAX as f64 /2.0 ;
        phenotype * phenotype
}
//This is possibly rendundant
fn score_vec(genes: &Vec<u32>) -> Vec<f64>{
    let mut scores: Vec<f64> =  Vec::new();
    for gene in genes.iter() {
        scores.push(score(*gene));
    }
    scores
}
fn mutate_pool(pool: &mut Vec<Sample>, rate: u32, rng: &mut ThreadRng){
    for sample in pool.iter_mut() {
        let seed:u32 =  rng.gen::<u32>();
        let p:u32 = seed % 100;
        if p < rate {
            let mut_dig:u8 = (seed % 32) as u8;
            let mutator:u32 = 1 << mut_dig;
            sample.gene = sample.gene ^ mutator;
            sample.score = score(sample.gene);
        }
    }
}
fn breed_pool(pool: &mut Vec<Sample>, rate: u32, rng: &mut ThreadRng){
    let n_breeders:usize = (pool.len() * rate as usize / 100) as usize;
    let mut children:Vec<Sample> = Vec::new();
    for i in 0..n_breeders {
        //Pick index of "mate"
        let mate_index:usize = rng.gen::<usize>() % pool.len();
        let middle = rng.gen::<u8>() % 32;
        let g1:u32 = pool[i].gene;
        let g2:u32 = pool[mate_index].gene;
        let head_g1 = (g1 >> middle) << middle;
        let head_g2 = (g2 >> middle) << middle;
        let tail_g1 = (g1 << middle) >> middle;
        let tail_g2 = (g2 << middle) >> middle;
        //make new samples
        let mut gene:u32 = head_g1 | tail_g2;
        let mut new_score:f64 = score(gene);
        children.push( Sample {
            gene: gene,
            score: new_score
        });
        gene = head_g2 | tail_g1;
        new_score = score(gene);
        children.push(Sample {
            gene: gene,
            score: new_score
        });
        }
    let pool_size = pool.len();
    let children_size = children.len();
    for i in 0..children_size {
        pool[pool_size-(i+1)] = children.pop().expect("Should be ok");
    }
    }
fn main() {
    //Parameters
    let mut n_generations: i32 = 10;
    let mut  n_samples: i32 = 10;
    let mut mut_rate:u32 = 5;
    let mut breed_rate:u32 = 5;
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
            "--mutationRate" => mut_rate = value as u32,
            "--breedRate" => breed_rate = value as u32,
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
        println!("Best is:   {}", pool[0].score);
        println!("Median is: {}", pool[(n_samples/2) as usize].score);
        println!("Worst is:  {}", pool[(n_samples - 1) as usize].score);

        let best_gene = pool[0].gene;
        let best_score = pool[0].score;
        breed_pool(&mut pool, mut_rate, &mut rng);
        mutate_pool(&mut pool, mut_rate, &mut rng);
        //pool[0] = Sample{
            //gene: best_gene,
            //score: best_score
        //};
    }
}
