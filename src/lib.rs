mod todo_list;
use todo_list::TodoList;

pub struct Config {
    function: String,
    params: Option<Vec<String>>,
}

impl Config {
    pub fn build(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() == 1 {
            return Ok(Config {
                function: String::from("list"),
                params: None,
            });
        }

        let mut args_clone = args.clone();
        args_clone.remove(0);
        Ok(Config {
            function: args_clone.remove(0),
            params: Some(args_clone),
        })
    }
}

pub fn run(config: Config) -> Result<(), &'static str> {
    let todo_list = match TodoList::build() {
        Ok(todo_list) => todo_list,
        Err(err) => return Err(err),
    };

    match config.function.as_str() {
        "add" => todo_list.add(),
        "remove" => todo_list.remove(),
        "done" => todo_list.done(),
        "list" => todo_list.list(),
        _ => unreachable!(),
    };

    Ok(())
}
