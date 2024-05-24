use rand::Rng;
use rand::rngs::ThreadRng;
use crate::get_params;
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
pub fn prisoners(params:get_params::Params, rng: &mut ThreadRng) {
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
                p.score += s1;
                let p:&mut Prisoner = &mut pool[k as usize];
                p.score += s2;
            }
        };
        //Calculate mean and std
        //Mate Players
        //Mutate Players
    }
}
// Method to calculate mean and std
fn mean(jail:&Vec<Prisoner>) -> (f64, f64) {
    let mean = jail.iter().map(|p| p.score).sum::<u64>() as f64/ (jail.len() as f64);
    let std = jail.iter().map(|p| (p.score as f64 - mean).powf(2.0)/ jail.len() as f64).sum::<f64>().sqrt();
    (mean, std)  
}
#[cfg(test)]
mod test_mean {
    use super::*;
    //Test mean calculated correctly
    #[test]
    fn test_1() {
       let jail:Vec<Prisoner> = vec![Prisoner {assumptions:0, strategy:0,score: 1}, Prisoner {assumptions:0, strategy:0, score: 2}, Prisoner {assumptions:0, strategy:0,score:3}]; 
       assert_eq!(mean(&jail).0, 2.0);
    }
    #[test]
    fn test_2() {
       let jail:Vec<Prisoner> = vec![Prisoner {assumptions:0, strategy:0,score: 2}, Prisoner {assumptions:0, strategy:0, score: 4}, Prisoner {assumptions:0, strategy:0,score:6}]; 
       assert_eq!(mean(&jail).0, 4.0);
    }
}
#[cfg(test)]
mod test_play {
    use super::*;
    //Test player that always cooperates against one that always defects
    #[test]
    fn test_play_1() {
        let mut p_0 = Prisoner {
            strategy: 0,
            assumptions: 0,
            score: 0
        };
        let mut p_1 = Prisoner {
            strategy: u64::MAX,
            assumptions: 0,
            score: 0
        };
        let result = play(&p_0, &p_1);
        assert_eq!(result.0, 0);
        assert_eq!(result.1, 50);
    }
    //Test that two identical players have same score
    #[test]
    fn test_identical_players() {
        for i in 0..300{
            let  p_0 = Prisoner {
                strategy: i,
                assumptions: 0,
                score: 0
            };
        let  p_1 = Prisoner {
            strategy: i,
            assumptions: 0,
            score: 0
        };
        let results = [play(&p_0, &p_1), play(&p_1, &p_0)];
        assert_eq!(results[0], results[1]);
        assert_eq!(results[0].0, results[0].1);
        }
    }
}

