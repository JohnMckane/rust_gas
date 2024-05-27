use std::env;
pub struct Params {
    pub n_generations: u32,
    pub n_samples: u32,
    pub mut_rate:u32,
    pub breed_rate:u32,
    pub problem:String
}
pub fn get_params() -> Params {
    let args: Vec<String> = env::args().collect();
    let mut params:Params = Params {
             n_generations: 10,
             n_samples: 10,
             mut_rate:10,
             breed_rate:10,
             problem: "minf(x)".to_string()
        };  
    //handle args
    for arg in args.iter() {
        let parts: Vec<&str> = arg.split(|c| c == '=').collect();
        if parts.len() < 2 {
            continue;
        }
        let setting = parts[0];
        let value = parts[1];

        match setting {
            "--generations" => params.n_generations = value.parse::<u32>().expect("Should be number"),
            "--samples" => params.n_samples = value.parse::<u32>().expect("Should be number"),
            "--mutationRate" => params.mut_rate = value.parse::<u32>().expect("Should be number"),
            "--breedRate" => params.breed_rate = value.parse::<u32>().expect("Should be number"),
            "--problem" => params.problem = value.to_string(),
            &_ => println!("Invalid Arg: {}", setting),
        }
    }
    params
}
