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
fn optimize_fx(params:get_params::Params, rng: &mut ThreadRng) {
    //Create Gene pool
    let mut pool:Vec<Sample> = Vec::new();
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
        breed_pool(&mut pool, params.mut_rate, rng);
        mutate_pool(&mut pool, params.mut_rate, rng);
        pool[0] = Sample{
            gene: best_gene,
            score: best_score
        };
    }
}
struct Prisoner {
    strategy:u64,
    assumptions: u8,
    score: u64
}
// Using cooperate = 0, defect = 1
fn play(p1:&Prisoner, p2:&Prisoner)-> (u64, u64){
    let mut p1_his:u8 = p1.assumptions & 0b00111111;
    let mut p2_his:u8 = p2.assumptions & 0b00111111;
    let mut p1_score = 0;
    let mut p2_score = 0;
    //Play p1 and p2 against each other 10 times.
    for _i in 0..10 {
        //Get both players next moves
        let p1_next:u8 = ((p1.strategy & ((1 as u64) << p1_his)) >> p1_his) as u8;
        let p2_next:u8 = ((p2.strategy & ((1 as u64) << p2_his)) >> p2_his) as u8;
        //Score Players
        if p1_next == 0 {
            if p2_next == 0 {
                p1_score += 3;
                p2_score += 3;
            } else {
                p2_score += 5;
            }
        } else {
            if p2_next == 0 {
                p2_score += 5;
            } else {
                p1_score += 1;
                p2_score += 1;
            }
        }
        //Add each players next move to history.
        //Use bitwise to treat history like two queues
       let front_mask:u8 = 0b00111000; 
       let back_mask:u8 =  0b00000111; 
       p1_his = p1_his >> 1;
       p1_his = ((p1_his & front_mask) & p1_next << 5) | ((p1_his & back_mask) & p2_next << 2);
       p2_his = p2_his >> 1;
       p2_his = ((p2_his & front_mask) & p2_next << 5) | ((p2_his & back_mask) & p1_next << 2);
    }
    (p1_score, p2_score)

}
fn prisoners(params:get_params::Params, rng: &mut ThreadRng) {
    //Initialize the pool (or jail)
    let mut pool:Vec<Prisoner> = Vec::new();
    for _i in 0..params.n_samples {
        pool.push(Prisoner {
            strategy: rng.gen::<u64>(),
            assumptions: rng.gen::<u8>(),
            score: 0
        });
    }
    //Run evolution process
    for _i in 0..params.n_generations {
        //Play each player against each other player, giving them scores
        for j in 0..params.n_samples-1 {
            for k in j+1..params.n_samples {
                let p1:&Prisoner = &pool[j as usize];
                let p2:&Prisoner = &pool[k as usize];
                let (s1, s2) = play(p1, p2);
                let p:&mut Prisoner = &mut pool[j as usize];
                p.score = s1;
                let p:&mut Prisoner = &mut pool[k as usize];
                p.score = s2;
            }
        };
        //Calculate mean and std
        //Mate Players
        //Mutate Players
    }
}
fn main() {
    let params = get_params::get_params();
    let mut rng = rand::thread_rng();
    if params.problem == "prisoners" {
        prisoners(params, &mut rng);
        return;
    }
    optimize_fx(params, &mut rng);
    
}
