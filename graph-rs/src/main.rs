use std::collections::HashMap;

mod lib;
use lib::Builder;

fn main() {
    println!("Computational Graph Library - Rust");
    println!("==========================================");

    // Run all examples
    example1();
    example2();
    example3();
    custom_example();
}

// Example 1: f(x) = x^2 + x + 5
fn example1() {
    println!("\nExample 1: f(x) = x^2 + x + 5");
    println!("-----------------------------");

    let mut builder = Builder::new();
    
    // Create nodes
    let x = builder.init();
    println!("Created input node x");
    
    let x_squared = builder.mul(x.clone(), x.clone());
    println!("Created x^2 node");
    
    let five = builder.constant(5);
    println!("Created constant node 5");
    
    let x_squared_plus_x = builder.add(x_squared, x);
    println!("Created (x^2 + x) node");
    
    let _result = builder.add(x_squared_plus_x, five);
    println!("Created result node (x^2 + x + 5)");
    
    // Test with x = 3
    let mut inputs = HashMap::new();
    inputs.insert(0, 3); // x = 3
    
    println!("\nFilling graph with x = 3");
    match builder.fill_nodes(inputs) {
        Ok(values) => {
            // Print all computed values
            println!("\nComputed values:");
            println!("x = {}", values.get(&0).unwrap_or(&0));
            println!("x^2 = {}", values.get(&1).unwrap_or(&0));
            println!("5 = {}", values.get(&2).unwrap_or(&0));
            println!("x^2 + x = {}", values.get(&3).unwrap_or(&0));
            println!("x^2 + x + 5 = {}", values.get(&4).unwrap_or(&0));
            
            // Check constraints
            println!("\nConstraints satisfied: {}", builder.check_constraints(&values));
        },
        Err(e) => println!("Error filling nodes: {}", e),
    }
}

// Example 2: f(a) = (a+1) / 8 (using hint for division)
fn example2() {
    println!("\nExample 2: f(a) = (a+1) / 8");
    println!("---------------------------");

    let mut builder = Builder::new();
    
    // Create nodes
    let a = builder.init();
    println!("Created input node a");
    
    let one = builder.constant(1);
    println!("Created constant node 1");
    
    let b = builder.add(a.clone(), one);
    println!("Created (a+1) node");
    
    let eight = builder.constant(8);
    println!("Created constant node 8");
    
    // Hint for division: c = b / 8
    let c = builder.hint(vec![b.clone()], |values| {
        let b_value = *values.get(&2).unwrap_or(&0);
        b_value / 8
    });
    println!("Created hint node (a+1)/8");
    
    // Constraint: c * 8 = b
    let c_times_8 = builder.mul(c.clone(), eight);
    println!("Created (c*8) node");
    
    builder.assert_equal(c_times_8, b);
    println!("Added constraint: c*8 = a+1");
    
    // Test with a = 15
    let mut inputs = HashMap::new();
    inputs.insert(0, 15); // a = 15
    
    println!("\nFilling graph with a = 15");
    match builder.fill_nodes(inputs) {
        Ok(values) => {
            // Print all computed values
            println!("\nComputed values:");
            println!("a = {}", values.get(&0).unwrap_or(&0));
            println!("1 = {}", values.get(&1).unwrap_or(&0));
            println!("a+1 = {}", values.get(&2).unwrap_or(&0));
            println!("8 = {}", values.get(&3).unwrap_or(&0));
            println!("(a+1)/8 = {}", values.get(&4).unwrap_or(&0));
            println!("((a+1)/8)*8 = {}", values.get(&5).unwrap_or(&0));
            
            // Check constraints
            println!("\nConstraints satisfied: {}", builder.check_constraints(&values));
        },
        Err(e) => println!("Error filling nodes: {}", e),
    }
}

// Example 3: f(x) = sqrt(x+7) (using hint for square root)
fn example3() {
    println!("\nExample 3: f(x) = sqrt(x+7)");
    println!("---------------------------");

    let mut builder = Builder::new();
    
    // Create nodes
    let x = builder.init();
    println!("Created input node x");
    
    let seven = builder.constant(7);
    println!("Created constant node 7");
    
    let x_plus_seven = builder.add(x, seven);
    println!("Created (x+7) node");
    
    // Hint for square root
    let sqrt_x_plus_7 = builder.hint(vec![x_plus_seven.clone()], |values| {
        let x_plus_seven_value = *values.get(&2).unwrap_or(&0);
        (x_plus_seven_value as f64).sqrt() as u32
    });
    println!("Created sqrt(x+7) node");
    
    // Constraint: sqrt_x_plus_7 * sqrt_x_plus_7 = x_plus_seven
    let computed_sq = builder.mul(sqrt_x_plus_7.clone(), sqrt_x_plus_7.clone());
    println!("Created (sqrt(x+7))^2 node");
    
    builder.assert_equal(computed_sq, x_plus_seven);
    println!("Added constraint: (sqrt(x+7))^2 = x+7");
    
    // Test with x = 2 (so x+7 = 9, sqrt = 3)
    let mut inputs = HashMap::new();
    inputs.insert(0, 2);
    
    println!("\nFilling graph with x = 2");
    match builder.fill_nodes(inputs) {
        Ok(values) => {
            // Print all computed values
            println!("\nComputed values:");
            println!("x = {}", values.get(&0).unwrap_or(&0));
            println!("7 = {}", values.get(&1).unwrap_or(&0));
            println!("x+7 = {}", values.get(&2).unwrap_or(&0));
            println!("sqrt(x+7) = {}", values.get(&3).unwrap_or(&0));
            println!("(sqrt(x+7))^2 = {}", values.get(&4).unwrap_or(&0));
            
            // Check constraints
            println!("\nConstraints satisfied: {}", builder.check_constraints(&values));
        },
        Err(e) => println!("Error filling nodes: {}", e),
    }
}

// Custom Example: f(x, y) = (x * y) + (x / y) (using hint for division)
fn custom_example() {
    println!("\nCustom Example: f(x, y) = (x * y) + (x / y)");
    println!("------------------------------------------");

    let mut builder = Builder::new();
    
    // Create input nodes
    let x = builder.init();
    println!("Created input node x");
    
    let y = builder.init();
    println!("Created input node y");
    
    // x * y
    let x_times_y = builder.mul(x.clone(), y.clone());
    println!("Created (x*y) node");
    
    // x / y (using hint)
    let x_div_y = builder.hint(vec![x.clone(), y.clone()], |values| {
        let x_value = *values.get(&0).unwrap_or(&0);
        let y_value = *values.get(&1).unwrap_or(&1); // Default to 1 to avoid division by zero
        
        // Avoid division by zero
        if y_value == 0 {
            return 0;
        }
        
        x_value / y_value
    });
    println!("Created (x/y) node");
    
    // Constraint: (x/y) * y = x
    let div_times_y = builder.mul(x_div_y.clone(), y.clone());
    println!("Created ((x/y)*y) node");
    
    // Add constraint: (x/y) * y = x
    builder.assert_equal(div_times_y, x.clone());
    println!("Added constraint: (x/y)*y = x");
    
    // Result: (x*y) + (x/y)
    let _result = builder.add(x_times_y, x_div_y);
    println!("Created result node (x*y)+(x/y)");
    
    // Test with x = 10, y = 2
    let mut inputs = HashMap::new();
    inputs.insert(0, 10); // x = 10
    inputs.insert(1, 2);  // y = 2
    
    println!("\nFilling graph with x = 10, y = 2");
    match builder.fill_nodes(inputs) {
        Ok(values) => {
            // Print all computed values
            println!("\nComputed values:");
            println!("x = {}", values.get(&0).unwrap_or(&0));
            println!("y = {}", values.get(&1).unwrap_or(&0));
            println!("x*y = {}", values.get(&2).unwrap_or(&0));
            println!("x/y = {}", values.get(&3).unwrap_or(&0));
            println!("(x/y)*y = {}", values.get(&4).unwrap_or(&0));
            println!("(x*y)+(x/y) = {}", values.get(&5).unwrap_or(&0));
            
            // Check constraints
            println!("\nConstraints satisfied: {}", builder.check_constraints(&values));
        },
        Err(e) => println!("Error filling nodes: {}", e),
    }
}

