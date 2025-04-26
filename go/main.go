package main

import (
	"fmt"
	"main/graph"
)

func main() {
	fmt.Println("Using Computational Graph Library")
	fmt.Println("========================================")

	example1()
}

// Example 1: f(x) = x^2 + x + 5
func example1() {
	fmt.Println("\nExample 1: f(x) = x^2 + x + 5")
	fmt.Println("-----------------------------")

	builder := graph.NewBuilder()

	// Create nodes
	x := builder.Init()
	fmt.Printf("Created input node x with ID: %d\n", x.ID)

	x_squared := builder.Mul(x, x)
	fmt.Printf("Created x^2 node with ID: %d\n", x_squared.ID)

	five := builder.Constant(5)
	fmt.Printf("Created constant node 5 with ID: %d\n", five.ID)

	x_squared_plus_x := builder.Add(x_squared, x)
	fmt.Printf("Created (x^2 + x) node with ID: %d\n", x_squared_plus_x.ID)

	result := builder.Add(x_squared_plus_x, five)
	fmt.Printf("Created result node (x^2 + x + 5) with ID: %d\n", result.ID)

	// Test with x = 3
	inputs := map[graph.NodeID]uint32{
		x.ID: 3, // x = 3
	}

	fmt.Println("\nFilling graph with x = 3")
	values, err := builder.FillNodes(inputs)
	if err != nil {
		fmt.Printf("Error filling nodes: %v\n", err)
		return
	}

	// Print all computed values
	fmt.Println("\nComputed values:")
	fmt.Printf("x = %d\n", values[x.ID])
	fmt.Printf("x^2 = %d\n", values[x_squared.ID])
	fmt.Printf("5 = %d\n", values[five.ID])
	fmt.Printf("x^2 + x = %d\n", values[x_squared_plus_x.ID])
	fmt.Printf("x^2 + x + 5 = %d\n", values[result.ID])

	// Check constraints
	fmt.Printf("\nConstraints satisfied: %v\n", builder.CheckConstraints(values))
}
