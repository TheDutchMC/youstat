use chrono::Datelike;

pub struct ProgramArgs {
    pub api_key:    Option<String>,
    pub input_file: Option<String>,
    pub year:       Option<u64>,
    pub max:       Option<u64>
}

impl Default for ProgramArgs {
    fn default() -> Self {
        Self {
            api_key: None,
            input_file: None,
            year: Some(chrono::Utc::now().year() as u64),
            max: Some(10)
        }
    }
}

impl ProgramArgs {
    pub fn is_valid(&self) -> (bool, &str) {
        if self.input_file.is_none() {
            (false, "Missing input file")
        } else if self.api_key.is_none() {
            (false, "Missing API key")
        } else {
            (true, "")
        }
    }

    pub fn new() -> Self {
        let args: Vec<String> = std::env::args().collect();
        let mut prog_args = ProgramArgs::default();
        for mut i in 0..args.len() {
            if let Some(arg) = args.get(i) {
                match arg.as_ref() {
                    "-i" => {
                        i+=1;
                        if let Some(arg) = args.get(i) {
                            prog_args.input_file = Some(arg.clone());
                        } else {
                            panic!("No value provided for argument '-i'");
                        }
                    },
                    "-a" => {
                        i+=1;
                        if let Some(arg) = args.get(i) {
                            prog_args.api_key = Some(arg.clone());
                        } else {
                            panic!("No value provided for argument '-a'");
                        }
                    },
                    "-y" => {
                        i+=1;
                        if let Some(arg) = args.get(i) {
                            let year = match arg.parse() {
                                Ok(y) => y,
                                Err(_e) => panic!("Value for argument '-y' is not numeric")
                            };

                            prog_args.year = Some(year);
                        } else {
                            panic!("No value provided for argument '-y'");
                        }
                    },
                    "-m" => {
                        i+=1;
                        if let Some(arg) = args.get(i) {
                            let max = match arg.parse() {
                                Ok(m) => m,
                                Err(_e) => panic!("Value for argument '-m' is not numeric")
                            };

                            prog_args.max = Some(max);
                        } else {
                            panic!("No value provided for argument '-m'");
                        }
                    }
                    _ => {}
                }
            }
        }

        prog_args
    }
}