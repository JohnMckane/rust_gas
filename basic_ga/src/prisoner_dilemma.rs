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
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(1,1);
    }
    #[test]
    fn test_2() {
        assert_eq!(2, 1);
    }
}

