use rand::Rng;
use rand::rngs::ThreadRng;
mod get_params;
mod basic;
mod prisoner_dilemma;

fn main() {
    let params = get_params::get_params();
    let mut rng = rand::thread_rng();
    if params.problem == "prisoners" {
        prisoner_dilemma::prisoners(params, &mut rng);
        return;
    }
    basic::optimize_fx(params, &mut rng);
    
}
