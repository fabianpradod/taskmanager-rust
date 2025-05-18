use std::collections::{HashMap, BinaryHeap}; // Imports HashMap and BinaryHeap
use std::io::{self, Write}; // Imports io and Write for input/output

#[derive(Debug, Clone)] // Allows cloning and printing of Task
struct Task {
    id: usize,                  // Id of Task
    description: String,        // Description of Task
    priority: u32,              // Priority of Task
    tags: Vec<String>,          // Tags associated with Task
}

struct TaskManager {
    tasks: Vec<Task>,                             // Vector of tasks
    queue: BinaryHeap<(u32, usize)>,              // Priority queue of tasks (priority, id)
    tag_index: HashMap<String, Vec<usize>>,       // Index of tags to task ids (tag, ids)
}

impl TaskManager {
    fn new() -> Self { // Constructor for TaskManager
        TaskManager {
            tasks: Vec::new(),                  // Initializes empty task vector
            queue: BinaryHeap::new(),           // Initializes empty priority queue
            tag_index: HashMap::new(),          // Initializes empty tag index
        }
    }

    fn add_task(&mut self, description: String, priority: u32, tags: Vec<String>) {
        // Adds a new task to the manager
        let id = self.tasks.len();
        let task = Task { id, description, priority, tags: tags.clone() };
        self.tasks.push(task);                // Inserts task into vector
        self.queue.push((priority, id));      // Inserts into priority queue

        for tag in tags {                     // For each tag
            self.tag_index.entry(tag)         // Takes tag as key
                .or_insert_with(Vec::new)
                .push(id);                    // Adds task id to tag index
        }
    }

    fn peek_next(&self) -> Option<Task> {
        // Show the next task without removing it
        self.queue.peek().map(|&(prio, id)| self.tasks[id].clone())
    }

    fn complete_next(&mut self) -> Option<Task> {
        // Remove and return the highest-priority task
        if let Some((_, id)) = self.queue.pop() {
            if let Some(task) = self.tasks.get(id).cloned() {
                self.tasks.retain(|t| t.id != id);  // Remove the task from the list
                self.rebuild();                     // Rebuild queue and tag index after changes
                return Some(task);
            }
        }
        None
    }

    fn list_tasks(&self) -> Vec<Task> {
        // Return a vector of tasks sorted by priority
        let mut list = self.tasks.clone();
        list.sort_by(|a, b| b.priority.cmp(&a.priority));
        list
    }

    fn rebuild(&mut self) {
        // Rebuilds the queue and tag index after a task is completed
        self.queue.clear();
        self.tag_index.clear();
        for (new_id, task) in self.tasks.iter_mut().enumerate() {
            task.id = new_id;
            self.queue.push((task.priority, new_id));
            for tag in &task.tags {
                self.tag_index.entry(tag.clone())
                    .or_insert_with(Vec::new)
                    .push(new_id);
            }
        }
    }

    fn tasks_by_tag(&self, tag: &str) -> Vec<&Task> {
        // Lists tasks by tag
        if let Some(ids) = self.tag_index.get(tag) {
            ids.iter()                                  // Iterates over task ids
               .filter_map(|&id| self.tasks.get(id))    // Gets task by id
               .collect()                               // Transforms to vector
        } else {
            Vec::new()                                  // Returns empty vector if no tasks found
        }
    }
}

fn main() {
    let mut manager = TaskManager::new(); // Creates a new TaskManager instance

    // Main program loop
    loop {
        println!("\n--- Task Manager ---");
        println!("1) Add task");
        println!("2) Show tasks by priority");
        println!("3) View next task");
        println!("4) Complete next task");
        println!("5) Search tasks by tag");
        println!("6) Exit");
        print!("Select an option: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                // Prompt for description
                print!("Description: ");
                io::stdout().flush().unwrap();
                let mut desc = String::new();
                io::stdin().read_line(&mut desc).unwrap();
                let desc = desc.trim().to_string();

                // Prompt for priority
                print!("Priority (integer): ");
                io::stdout().flush().unwrap();
                let mut prio = String::new();
                io::stdin().read_line(&mut prio).unwrap();
                let prio: u32 = prio.trim().parse().unwrap_or(0);

                // Prompt for tags
                print!("Tags (comma-separated): ");
                io::stdout().flush().unwrap();
                let mut tags_line = String::new();
                io::stdin().read_line(&mut tags_line).unwrap();
                let tags: Vec<String> = tags_line
                    .trim()
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();

                manager.add_task(desc, prio, tags);
                println!("Task added.");
            }
            "2" => {
                // Display all tasks sorted by priority
                println!("\nTasks by priority:");
                for task in manager.list_tasks() {
                    println!("{:?}", task);
                }
            }
            "3" => {
                // Show the next task without removing it
                if let Some(task) = manager.peek_next() {
                    println!("Next task: {:?}", task);
                } else {
                    println!("No tasks available.");
                }
            }
            "4" => {
                // Complete and remove the next task
                if let Some(task) = manager.complete_next() {
                    println!("Completed task: {:?}", task);
                } else {
                    println!("No tasks to complete.");
                }
            }
            "5" => {
                // Prompt for tag to search
                print!("Enter tag: ");
                io::stdout().flush().unwrap();
                let mut tag = String::new();
                io::stdin().read_line(&mut tag).unwrap();
                let tag = tag.trim();

                // Search and display matching tasks
                let results = manager.tasks_by_tag(tag);
                if results.is_empty() {
                    println!("No tasks found for tag '{}'.", tag);
                } else {
                    println!("Tasks with tag '{}':", tag);
                    for task in results {
                        println!("{:?}", task);
                    }
                }
            }
            "6" => break,
            _   => println!("Invalid option."),            
        }
    }
}
