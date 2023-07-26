#[derive(Debug)]
pub enum OptionCode {
    Add,
    Remove,
    List,
    Move,
    Dash,
    Help,
    CommandError,
}

pub fn get_opt(args: &Vec<String>) -> OptionCode {
    if args.len() <= 1 {
        return OptionCode::Help
    }

    match args[1].as_str() {
        "a" => {
            if args.len() != 2 {
                OptionCode::CommandError
            }else {
                OptionCode::Add
            }
        },

        "rm" => {
            if args.len() != 3 {
                OptionCode::CommandError
            }else{
                OptionCode::Remove
            }
        },

        "ls" => {
            if args.len() > 3 {
                OptionCode::CommandError
            }else{
                OptionCode::List
            }
        },

        "mv" => {
            if args.len() != 4 {
                OptionCode::CommandError
            }else{
                OptionCode::Move
            }
        },

        "h" => OptionCode::Help,
        _ => OptionCode::Dash,
}
}

