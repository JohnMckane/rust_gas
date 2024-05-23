use rand::Rng;
use rand::rngs::ThreadRng;
mod get_params;

//Sample struct contains samples gene and score
struct Sample {
    gene: u32,
    score: f32
}
fn score(gene: u32) -> f32 {
        let phenotype = (gene as f32 - u32::MAX as f32 /2.0) / (0.125 * u32::MAX as f32) ;
        -phenotype * (phenotype * 10.0 * 3.141592).sin() + 1.0
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
        let mut new_score:f32 = score(gene);
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
fn optimize_fx(params:get_params::Params) {
let mut rng = rand::thread_rng();
    //Create Gene pool
    let mut pool: Vec<Sample> = Vec::new();
    for _i in 0..params.n_samples {
        let gene:u32 = rng.gen::<u32>();
        pool.push(Sample {
            gene: gene,
            score: score(gene),
        });
    }
    //Evolution Process.
    for i in 0..params.n_generations {
        //sort samples by score
        pool.sort_by(|s_1, s_2| s_1.score.partial_cmp(&s_2.score).unwrap());
        if i % 15 == 0 {
        println!("Best is:   {}", pool[0].score);
        println!("Median is: {}", pool[(params.n_samples/2) as usize].score);
        println!("Worst is:  {}", pool[(params.n_samples - 1) as usize].score);
        }

        let best_gene = pool[0].gene;
        let best_score = pool[0].score;
        breed_pool(&mut pool, params.mut_rate, &mut rng);
        mutate_pool(&mut pool, params.mut_rate, &mut rng);
        pool[0] = Sample{
            gene: best_gene,
            score: best_score
        };
    }
}
fn prisoners(params:get_params::Params) {
}
fn main() {
    let params = get_params::get_params();
    if params.problem == "prisoners" {
        prisoners(params);
        return;
    }
    optimize_fx(params);
    
}
