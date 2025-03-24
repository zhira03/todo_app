mod tasks;

use clap::{Arg, Command};
use tasks::{Task, save_tasks, load_tasks,delete_task};

fn main(){
    let matches = Command::new("To-Do C_L_I App")
        .version("1.0.0.1")
        .about("A Simple To Do App")
        .subcommand(Command::new("add")
            .about("Add a new Task")
            .arg(Arg::new("description")
                .required(true)
                .help("Description of the Task")))
        .subcommand(Command::new("list").about("List All Tasks"))
        .subcommand(Command::new("complete")
            .about("Mark Task as Completed")
            .arg(Arg::new("id")
                .required(true)
                .help("ID of the Task to be Completed")))
        .subcommand(Command::new("delete")
            .about("Delete a Task")
            .arg(Arg::new("id")
                .required(true)
                .help("ID of the Task to be Deleted")))
        .get_matches();

    let mut tasks = load_tasks();

    if let Some(add_matches) = matches.subcommand_matches("add"){
        let description = add_matches.get_one::<String>("description").unwrap();
        let id = tasks.len() as u32 +1;
        tasks.push(Task::new(id, description.clone()));
        save_tasks(&tasks).expect("Failed to save Task");
        println!("Task Added: {description}");
    }else if matches.subcommand_matches("list").is_some(){
        if tasks.is_empty(){
            println!("No Tasks Just Yet Bro");
        } else {
            for task in &tasks{
                let status = if task.completed { "[✔ ]" } else { "[  ]" };
                println!("{status} {} - {}", task.id, task.description);
            }
        }
    }else if let Some(complete_matches) = matches.subcommand_matches("complete") {
        let id = complete_matches.get_one::<String>("id").unwrap().parse::<u32>().unwrap();
        if let Some(task) = tasks.iter_mut().find(|task| task.id == id){
            task.completed = true;
            save_tasks(&tasks).expect("Failed to save Task");
            println!("Task Completed: {id}");
        } else {
            println!("Task with ID {id} not found");
        }
    }else if let Some(delete_matches) = matches.subcommand_matches("delete"){
        if let Ok(id) = delete_matches.get_one::<String>("id").unwrap().parse::<u32>(){
            if delete_task(&mut tasks, id){
                println!("Task Deleted: {id}");
            }else {
                println!("Task with ID {id} not Found");
            }
        }else {
            println!("Invalid ID format. Bro Enter an Acyual Number init");
        }
    }
}

