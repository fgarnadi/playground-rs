use crate::storage;
use crate::task::{Priority, Task};

pub fn add_task(title: String, priority: Priority) -> Result<(), Box<dyn std::error::Error>> {
    let mut tasks = storage::load_tasks()?;
    let id = tasks.last().map_or(1, |t| t.id + 1);
    let task = Task::new(id, title.clone(), priority);
    println!("Added task: {task}");
    tasks.push(task);
    storage::save_tasks(&tasks)?;
    Ok(())
}

pub fn list_tasks(show_done: bool) -> Result<(), Box<dyn std::error::Error>> {
    let tasks = storage::load_tasks()?;
    let filtered: Vec<&Task> = tasks.iter().filter(|t| show_done || !t.done).collect();

    if filtered.is_empty() {
        println!("No tasks! 🎉");
        return Ok(());
    }

    for task in filtered.iter() {
        println!("  {task}");
    }

    println!("\n{} task(s) shown", filtered.len());
    Ok(())
}

pub fn complete_task(id: u32) -> Result<(), Box<dyn std::error::Error>> {
    let mut tasks = storage::load_tasks()?;
    let task = tasks
        .iter_mut()
        .find(|t| t.id == id)
        .ok_or(format!("No task with id {id}"))?;

    task.done = true;
    println!("Completed: {task}");

    storage::save_tasks(&tasks)?;
    Ok(())
}

pub fn remove_task(id: u32) -> Result<(), Box<dyn std::error::Error>> {
    let mut tasks = storage::load_tasks()?;
    let len_before = tasks.len();

    tasks.retain(|t| t.id != id);
    if tasks.len() == len_before {
        return Err(format!("No task with id {id}").into());
    }
    println!("Removed task {id}");

    storage::save_tasks(&tasks)?;
    Ok(())
}

pub fn show_stats() -> Result<(), Box<dyn std::error::Error>> {
    let tasks = storage::load_tasks()?;
    let total = tasks.len();
    let done = tasks.iter().filter(|t| t.done).count();
    let pending = total - done;

    let counts = tasks
        .iter()
        .fold(std::collections::HashMap::new(), |mut acc, t| {
            *acc.entry(&t.priority).or_insert(0) += 1;
            acc
        });

    let high = counts.get(&Priority::High).unwrap_or(&0);
    let medium = counts.get(&Priority::Medium).unwrap_or(&0);
    let low = counts.get(&Priority::Low).unwrap_or(&0);

    println!("📊 Task Statistics");
    println!("   Total:   {total}");
    println!("   Done:    {done} ✅");
    println!("   Pending: {pending}");
    println!("   🔴 High:   {high}");
    println!("   🟡 Medium: {medium}");
    println!("   🟢 Low:    {low}");

    Ok(())
}
