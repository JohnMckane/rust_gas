use rand::Rng;
use rand::rngs::ThreadRng;
use crate::get_params;
#[derive(Copy)]
#[derive(Clone)]
#[derive(PartialEq)]
struct Prisoner {
    strategy:u64,
    assumptions: u8,
    score: f64,
    history: u8
}
impl Prisoner {
    fn new_blank() -> Prisoner {
        Prisoner {
            strategy: 0,
            assumptions: 0,
            score: 0.0,
            history: 0
        }
    }
    fn new(strategy:u64, assumptions:u8 ) -> Prisoner {
        Prisoner {
            strategy: strategy,
            assumptions: assumptions,
            score: 0.0,
            history:0
        }
    }
    fn init_history(&mut self) {
        self.history = self.assumptions & 0b00111111;
    }
}
// Using cooperate = 0, defect = 1
fn play( jail:&mut Vec<Prisoner>, i:usize, j:usize)-> (f64, f64){
    jail[i].init_history();
    jail[j].init_history();
    //Play both players against each other 10 times.
    for _i in 0..10 {
        //Get both players next moves
        let p1_next:u8 = ((jail[i].strategy & ((1 as u64) << jail[i].history)) >> jail[i].history) as u8;
        let p2_next:u8 = ((jail[j].strategy & ((1 as u64) << jail[j].history)) >> jail[j].history) as u8;
        //Score Players
        if p1_next == 0 {
            if p2_next == 0 {
                jail[i].score += 3.0;
                jail[j].score += 3.0;
            } else {
                jail[j].score += 5.0;
            }
        } else {
            if p2_next == 0 {
                jail[j].score += 5.0;
            } else {
                jail[i].score += 1.0;
                jail[j].score += 1.0;
            }
        }
        //Add each players next move to history.
        //Use bitwise to treat history like two queues
       let front_mask:u8 = 0b00111000; 
       let back_mask:u8 =  0b00000111; 
       jail[i].history = jail[i].history >> 1;
       jail[i].history = ((jail[i].history & front_mask) & p1_next << 5) | ((jail[i].history & back_mask) & p2_next << 2);
       jail[j].history = jail[j].history >> 1;
       jail[j].history = ((jail[j].history & front_mask) & p2_next << 5) | ((jail[j].history & back_mask) & p1_next << 2);
    }
    (jail[i].score, jail[j].score)

}
pub fn prisoners(params:get_params::Params, rng: &mut ThreadRng) {
    //Initialize the pool (or jail)
    let mut pool:Vec<Prisoner> = Vec::new();
    for _i in 0..params.n_samples {
        pool.push(Prisoner::new(rng.gen::<u64>(), rng.gen::<u8>()));
    }
    //Run evolution process
    for _i in 0..params.n_generations {
        println!("{}", pool.len());
        //Play each player against each other player, giving them scores
        if pool.len() == 0  {
            break;
        }
        for j in 0..pool.len()-1 {
            for k in j+1..pool.len() {
                play(&mut pool, j, k);
            }
        };
        //Cull bottom players
        pool.sort_by_key(|p| - (p.score as i32));
        while pool.len() > params.n_samples as usize {
            pool.pop();
        }
        //Save top 3, re insert at bottom of loop.
        let mut b3:Vec<Prisoner> = vec![pool[0], pool[1], pool[2], pool[3], pool[4], pool[5]];
        println!("Best:   {}" , pool[0].score);
        println!("Wosrst: {}" , pool[pool.len() -1].score);
        //Calculate mean and std
        let (mean, std) = mean(&pool);
        println!("mean: {}", mean);
        println!("std : {}", std);
        let mut mate_trac:Vec<(Prisoner, u8)> = Vec::new();
        //Mate Players
        while pool.len() > 0 {
            let p = pool.pop().unwrap();
            let mut mate_chances:u8 = 0; 
            if p.score as f64 > mean -  std {
                mate_chances += 1;
            }
            if p.score as f64 > mean + std {
                mate_chances += 1;
            }

            mate_trac.push((p , mate_chances));
        }
        mate_trac.sort_by_key(|p| - (p.1 as i8));
        while mate_trac.len() > 0 {
            let mut p = mate_trac.pop().unwrap();
            if p.1 == 0 || mate_trac.len() == 0{
                continue;
            }
            p.1 -= 1;
            // Pick random pairing
            let mate_index =rng.gen::<usize>() % mate_trac.len();
            mate_trac[mate_index].1 -=1;
            let mate = &mate_trac[mate_index];
            let ass_mask = rng.gen::<u8>();
            let str_mask = rng.gen::<u64>();
            let children = (Prisoner::new((p.0.strategy & str_mask ) | (mate.0.strategy & !str_mask), (p.0.assumptions & ass_mask )| (mate.0.assumptions & !ass_mask)), Prisoner::new((mate.0.strategy & str_mask )| (p.0.strategy & !str_mask),(mate.0.assumptions & ass_mask )| (p.0.assumptions & !ass_mask)));
            pool.push(children.0);
            pool.push(children.1);
            mate_trac.sort_by_key(|p|- (p.1 as i8));
        }
        //Mutate step, for each 'chromosone' of sample to be mutated, use xor of random number.
        for prisoner in pool.iter_mut() {
            //Set score to 0
                (*prisoner).score = 0.0;

            if rng.gen::<u32>() % 100 > params.mut_rate  {
                continue;
            }
            (*prisoner).strategy = prisoner.strategy  ^ (rng.gen::<u64>() &  rng.gen::<u64>() &  rng.gen::<u64>());
            (*prisoner).assumptions = prisoner.assumptions ^ (rng.gen::<u8>() & rng.gen::<u8>() & rng.gen::<u8>());
        }
        while b3.len() > 0 {
        let mut p = b3.pop().expect("not found");
        p.score = 0.0;
        if p.score > 0.0 {
                println!("score not reset after pop");
                break;
            }
        pool.push(p);
        }
        for p in pool.iter(){
            if p.score > 0.0 {
                println!("score not reset");
                break;
            }
        }
    }
}
// Method to calculate mean and std
fn mean(jail:&Vec<Prisoner>) -> (f64, f64) {
    let mean = jail.iter().map(|p| p.score).sum::<f64>() / (jail.len() as f64);
    let std = jail.iter().map(|p| (p.score as f64 - mean).powf(2.0)/ jail.len() as f64).sum::<f64>().sqrt();
    (mean, std)  
}
#[cfg(test)]
mod test_mean {
    use super::*;
    //Test mean calculated correctly
    #[test]
    fn test_mean_1() {
       let jail:Vec<Prisoner> = gen_jail(vec![1.0, 2.0, 3.0]);
       assert_eq!(mean(&jail).0, 2.0);
    }
    #[test]
    fn test_mean_2() {
       let jail:Vec<Prisoner> = gen_jail(vec![2.0, 4.0, 6.0]);
       assert_eq!(mean(&jail).0, 4.0);
    }
    #[test]
    fn test_std() {
        let jail:Vec<Prisoner> = gen_jail(vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0]);
       assert_eq!(mean(&jail), (5.0, 2.0));
    }
    fn gen_jail(scores:Vec<f64>) -> Vec<Prisoner> {
        let mut jail:Vec<Prisoner> = Vec::new();
        for s in scores.iter() {
            jail.push(Prisoner {assumptions:0, strategy:0,score: *s, history: 0});
        }
        jail
    }
}
#[cfg(test)]
mod test_play {
    use super::*;
    //Test player that always cooperates against one that always defects
    #[test]
    fn test_play_1() {
        let mut jail:Vec<Prisoner> = vec!{Prisoner::new(0, 0),
            Prisoner::new(u64::MAX, 0)};
        let result = play(&mut jail, 0, 1);
        assert_eq!(result.0, 0.0);
        assert_eq!(result.1, 50.0);
    }
    //Test that two identical players have same score
    #[test]
    fn test_identical_players() {
        for i in 0..300{
            let mut jail:Vec<Prisoner> = vec!{Prisoner::new(i, 0),
                Prisoner::new(i, 0)};
            let result = play(&mut jail, 0, 1);
            assert_eq!(result.0, result.1);
        }
    }
}

