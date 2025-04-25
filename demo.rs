use std::thread;
use std::time::Duration;

// This is a simple demo of how the hardware and software components work together
fn main() {
    println!("=== Laptop System Demo ===");
    println!("This demo shows how hardware and software components interact.");
    println!("\nWe've implemented several components:");
    println!("1. Hardware components (CPU, GPU, memory, storage, etc.)");
    println!("2. Software components (OS, applications, drivers, etc.)");
    println!("3. Visualizations (external view, hardware view, software flow)");
    
    println!("\nHardware provides the foundation for software to run.");
    println!("Software relies on hardware to execute instructions.");
    println!("When software sends instructions, hardware processes them and responds.");
    
    println!("\nIn our visualization system, we can see:");
    println!("- The exterior of the laptop (keyboard, screen, etc.)");
    println!("- The internal hardware components");
    println!("- Software flowing through hardware like a soul through a body");
    
    // Simulate some interactions
    println!("\n[Demo] Simulating system interactions...");
    for i in 1..=5 {
        println!("\n--- Interaction {} ---", i);
        
        // Simulate a command
        let command = match i {
            1 => "open browser",
            2 => "check memory usage",
            3 => "list files in /documents",
            4 => "run calculator",
            5 => "close all applications",
            _ => "unknown command",
        };
        
        println!("[User] Command: \"{}\"", command);
        
        // Simulate processing
        println!("[Software] Processing command...");
        thread::sleep(Duration::from_millis(300));
        println!("[Software] Converting to binary instructions...");
        thread::sleep(Duration::from_millis(200));
        
        println!("[Hardware] CPU receiving instructions...");
        thread::sleep(Duration::from_millis(200));
        println!("[Hardware] Processing data...");
        thread::sleep(Duration::from_millis(300));
        
        if i == 1 {
            println!("[Hardware] GPU rendering browser interface...");
            thread::sleep(Duration::from_millis(400));
        } else if i == 2 {
            println!("[Hardware] Reading memory usage data...");
            println!("[Hardware] Memory usage: 2.4GB / 8GB");
            thread::sleep(Duration::from_millis(200));
        } else if i == 3 {
            println!("[Hardware] Reading from storage...");
            println!("[Hardware] Found 15 files in /documents");
            thread::sleep(Duration::from_millis(300));
        }
        
        println!("[Software] Processing hardware response...");
        thread::sleep(Duration::from_millis(200));
        println!("[Software] Updating user interface...");
        thread::sleep(Duration::from_millis(300));
        
        let result = match i {
            1 => "Browser opened successfully.",
            2 => "Memory usage: 30% (2.4GB / 8GB)",
            3 => "15 files found in /documents",
            4 => "Calculator application started.",
            5 => "All applications closed.",
            _ => "Unknown command.",
        };
        
        println!("[System] Result: {}", result);
        
        // Artificial delay between interactions
        thread::sleep(Duration::from_secs(1));
    }
    
    println!("\n=== Demo Complete ===");
    println!("This demonstrates how software instructions flow through hardware components.");
    println!("The full implementation allows visualizing this flow in 3D.");
} 