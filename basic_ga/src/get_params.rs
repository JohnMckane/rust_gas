use std::env;
pub struct Params {
    pub n_generations: u32,
    pub n_samples: u32,
    pub mut_rate:u32,
    pub breed_rate:u32
}
pub fn get_params() -> Params {
    let args: Vec<String> = env::args().collect();
    let mut params:Params = Params {
             n_generations: 10,
             n_samples: 10,
             mut_rate:10,
             breed_rate:10
        };  
    //handle args
    for arg in args.iter() {
        let parts: Vec<&str> = arg.split(|c| c == '=').collect();
        if parts.len() < 2 {
            continue;
        }
        let setting = parts[0];
        let value:u32 = parts[1].parse::<u32>().expect("Should"); 

        match setting {
            "--generations" => params.n_generations = value,
            "--samples" => params.n_samples = value,
            "--mutationRate" => params.mut_rate = value,
            "--breedRate" => params.breed_rate = value,
            &_ => println!("Invalid Arg: {}", setting),
        }
    }
    params
}
