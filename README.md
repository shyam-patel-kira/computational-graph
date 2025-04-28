# Computational Graph Library

This repository contains implementations of a computational graph library in both Rust and Go, as specified in the Succinct ZK Takehome assignment.

## Overview

The library allows users to define a computational graph with nodes related by addition and multiplication operations. It supports:

- Creating input nodes and constant nodes
- Performing addition and multiplication operations between nodes
- Asserting equality constraints between nodes
- Filling in the graph based on input node values
- Checking that all equality constraints are satisfied
- Hinting arbitrary node values based on other nodes' values

## Implementation Details

### Core Components

1. **Node**: Represents a value in the computational graph
   - Input nodes: Require values to be provided
   - Constant nodes: Have fixed values
   - Operation nodes: Derived from addition or multiplication
   - Hint nodes: Computed outside the graph but constrained within it

2. **Builder**: Manages the graph construction and evaluation
   - Creates nodes
   - Establishes relationships between nodes
   - Fills in node values
   - Checks constraints

3. **Constraints**: Equality assertions between nodes

4. **Hint Mechanism**: Allows for operations outside the graph (like division or square root)

### Rust Implementation

The Rust implementation uses:
- Strong typing with enums for node types
- Ownership model for safe memory management
- Arc (atomic reference counting) for shared ownership of hint functions
- Error handling with Result types
- Comprehensive testing for all examples

### Go Implementation

The Go implementation uses:
- Interface-based design
- Maps for efficient node lookup
- First-class functions for the hint mechanism
- Iterative graph filling algorithm
- Comprehensive testing for all examples

## Examples

The library supports various computational graphs, including:

1. **Example 1**: f(x) = x² + x + 5
2. **Example 2**: f(a) = (a+1) / 8 (using hint for division)
3. **Example 3**: f(x) = sqrt(x+7) (using hint for square root)

## Testing

Both implementations include comprehensive test cases for all three examples, ensuring that:
- Graphs are constructed correctly
- Values are computed accurately
- Constraints are properly enforced
- Hint mechanism works as expected

## Usage

### Rust

```rust
// Example: f(x) = x² + x + 5
let mut builder = Builder::new();
let x = builder.init();
let x_squared = builder.mul(x.clone(), x.clone());
let five = builder.constant(5);
let x_squared_plus_x = builder.add(x_squared, x);
let result = builder.add(x_squared_plus_x, five);

// Fill in the graph with x = 3
let mut inputs = HashMap::new();
inputs.insert(0, 3);
let values = builder.fill_nodes(inputs).unwrap();

// Check the result
assert_eq!(values.get(&4), Some(&17)); // 3² + 3 + 5 = 17
assert!(builder.check_constraints(&values));
```

### Go

```go
// Example: f(x) = x² + x + 5
builder := NewBuilder()
x := builder.Init()
x_squared := builder.Mul(x, x)
five := builder.Constant(5)
x_squared_plus_x := builder.Add(x_squared, x)
result := builder.Add(x_squared_plus_x, five)

// Fill in the graph with x = 3
inputs := map[NodeID]uint32{
    x.ID: 3,
}
values, _ := builder.FillNodes(inputs)

// Check the result
if values[result.ID] != 17 { // 3² + 3 + 5 = 17
    // Error
}
if !builder.CheckConstraints(values) {
    // Error
}
```

## Future Improvements

Potential enhancements for the library:
- Support for more operations (subtraction, division, etc.)
- Optimization for large graphs
- Visualization tools for debugging
- Serialization/deserialization of graphs
- Parallel evaluation of independent nodes

# Computational Graph Library

This repo contains my implementation of a computational graph library in Go. I've also  translated it to Rust with what knowledge I had and with help of some AI tools.

## Overview

I've built a library that lets you create and work with computational graphs. As described in the instruction think of it as building a circuit where values flow from inputs through various operations like addition and multiplication.

I've also added support for defining special constraints and "hint" values for operations that aren't directly supported (like division or square root).

## What can it do?

- Create input nodes (where you provide values) and constant nodes (fixed values)
- Connect nodes with add and multiply operations
- Assert that certain nodes must be equal (constraints)
- Fill in all values in the graph once you provide the inputs
- Check if all your constraints are satisfied
- Use "hints" to work around limitations (like performing division when you only have multiplication)

## Design

I've kept the design pretty straightforward as what I've gathered from instructions:

- **Nodes** represent values in the graph (inputs, constants, or results of operations)
- **Builders** help you construct and evaluate the graph
- **Constraints** let you specify that different nodes must have the same value
- **Hints** allow you to compute values outside the graph but keep them constrained correctly

Both Rust and Go have similar APIs.

## Examples

I've included the three examples provided in the instructions to show how it works:

1. **Basic arithmetic**: `f(x) = x² + x + 5`
2. **Division using hints**: `f(a) = (a+1) / 8`
3. **Square root using hints**: `f(x) = sqrt(x+7)`

To see how they work, I've also created `main.go` and `main.rs` files.

## Try it yourself

### In Go:

To directly run `main.go` file just run the following cmd in `graph-go` folder
```bash
go run main.go
```
For running tests just run the following cmd in `graph-go` folder
```bash
go test -v ./...
```

To use it in your code:
```go
// Calculate x² + x + 5
builder := graph.NewBuilder()
x := builder.Init()
x_squared := builder.Mul(x, x)
five := builder.Constant(5)
x_squared_plus_x := builder.Add(x_squared, x)
result := builder.Add(x_squared_plus_x, five)

// Set x = 3 and evaluate
inputs := map[graph.NodeID]uint32{x.ID: 3}
values, _ := builder.FillNodes(inputs)

// Result should be 17 (3² + 3 + 5)
fmt.Printf("Result: %d\n", values[result.ID])
```

### In Rust:
To directly run `main.rs` file just run the following cmd in `graph-rs` folder
```bash
cargo run
```
For running tests just run the following cmd in `graph-rs` folder
```bash
cargo test
```

To use it in your code
```rust
// Same calculation in Rust
let mut builder = Builder::new();
let x = builder.init();
let x_squared = builder.mul(x.clone(), x.clone());
let five = builder.constant(5);
let x_squared_plus_x = builder.add(x_squared, x.clone());
let result = builder.add(x_squared_plus_x, five);

// Set x = 3 and evaluate
let mut inputs = HashMap::new();
inputs.insert(0, 3);
let values = builder.fill_nodes(inputs).unwrap();

// Result should be 17
println!("Result: {}", values.get(&4).unwrap());
```

## Testing

I've written tests for all the main functionality, including the example functions. Although it's a bit redundant, I've written unit tests and also included a main file that uses the same examples. 