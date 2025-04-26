package main

import (
	"fmt"
	"main/graph"
	"math"
)

func main() {
	fmt.Println("Using Computational Graph Library")
	fmt.Println("========================================")

	// Run all examples
	example1()
	example2()
	example3()
	customExample()
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

// Example 2: f(a) = (a+1) / 8 (using hint for division)
func example2() {
	fmt.Println("\nExample 2: f(a) = (a+1) / 8")
	fmt.Println("---------------------------")

	builder := graph.NewBuilder()

	// Create nodes
	a := builder.Init()
	fmt.Printf("Created input node a with ID: %d\n", a.ID)

	one := builder.Constant(1)
	fmt.Printf("Created constant node 1 with ID: %d\n", one.ID)

	b := builder.Add(a, one)
	fmt.Printf("Created (a+1) node with ID: %d\n", b.ID)

	eight := builder.Constant(8)
	fmt.Printf("Created constant node 8 with ID: %d\n", eight.ID)

	// Hint for division: c = b / 8
	c := builder.Hint([]*graph.Node{b}, func(values map[graph.NodeID]uint32) uint32 {
		return values[b.ID] / 8
	})
	fmt.Printf("Created hint node (a+1)/8 with ID: %d\n", c.ID)

	// Constraint: c * 8 = b
	c_times_8 := builder.Mul(c, eight)
	fmt.Printf("Created (c*8) node with ID: %d\n", c_times_8.ID)

	builder.AssertEqual(c_times_8, b)
	fmt.Println("Added constraint: c*8 = a+1")

	// Test with a = 15
	inputs := map[graph.NodeID]uint32{
		a.ID: 15, // a = 15
	}

	fmt.Println("\nFilling graph with a = 15")
	values, err := builder.FillNodes(inputs)
	if err != nil {
		fmt.Printf("Error filling nodes: %v\n", err)
		return
	}

	// Print all computed values
	fmt.Println("\nComputed values:")
	fmt.Printf("a = %d\n", values[a.ID])
	fmt.Printf("1 = %d\n", values[one.ID])
	fmt.Printf("a+1 = %d\n", values[b.ID])
	fmt.Printf("8 = %d\n", values[eight.ID])
	fmt.Printf("(a+1)/8 = %d\n", values[c.ID])
	fmt.Printf("((a+1)/8)*8 = %d\n", values[c_times_8.ID])

	// Check constraints
	fmt.Printf("\nConstraints satisfied: %v\n", builder.CheckConstraints(values))
}

// Example 3: f(x) = sqrt(x+7) (using hint for square root)
func example3() {
	fmt.Println("\nExample 3: f(x) = sqrt(x+7)")
	fmt.Println("---------------------------")

	builder := graph.NewBuilder()

	// Create nodes
	x := builder.Init()
	fmt.Printf("Created input node x with ID: %d\n", x.ID)

	seven := builder.Constant(7)
	fmt.Printf("Created constant node 7 with ID: %d\n", seven.ID)

	x_plus_seven := builder.Add(x, seven)
	fmt.Printf("Created (x+7) node with ID: %d\n", x_plus_seven.ID)

	// Hint for square root
	sqrt_x_plus_7 := builder.Hint([]*graph.Node{x_plus_seven}, func(values map[graph.NodeID]uint32) uint32 {
		return uint32(math.Sqrt(float64(values[x_plus_seven.ID])))
	})
	fmt.Printf("Created sqrt(x+7) node with ID: %d\n", sqrt_x_plus_7.ID)

	// Constraint: sqrt_x_plus_7 * sqrt_x_plus_7 = x_plus_seven
	computed_sq := builder.Mul(sqrt_x_plus_7, sqrt_x_plus_7)
	fmt.Printf("Created (sqrt(x+7))^2 node with ID: %d\n", computed_sq.ID)

	builder.AssertEqual(computed_sq, x_plus_seven)
	fmt.Println("Added constraint: (sqrt(x+7))^2 = x+7")

	// Test with x = 2 (so x+7 = 9, sqrt = 3)
	inputs := map[graph.NodeID]uint32{
		x.ID: 2,
	}

	fmt.Println("\nFilling graph with x = 2")
	values, err := builder.FillNodes(inputs)
	if err != nil {
		fmt.Printf("Error filling nodes: %v\n", err)
		return
	}

	// Print all computed values
	fmt.Println("\nComputed values:")
	fmt.Printf("x = %d\n", values[x.ID])
	fmt.Printf("7 = %d\n", values[seven.ID])
	fmt.Printf("x+7 = %d\n", values[x_plus_seven.ID])
	fmt.Printf("sqrt(x+7) = %d\n", values[sqrt_x_plus_7.ID])
	fmt.Printf("(sqrt(x+7))^2 = %d\n", values[computed_sq.ID])

	// Check constraints
	fmt.Printf("\nConstraints satisfied: %v\n", builder.CheckConstraints(values))
}

// Custom Example: f(x, y) = (x * y) + (x / y) (using hint for division)
func customExample() {
	fmt.Println("\nCustom Example: f(x, y) = (x * y) + (x / y)")
	fmt.Println("------------------------------------------")

	builder := graph.NewBuilder()

	// Create input nodes
	x := builder.Init()
	fmt.Printf("Created input node x with ID: %d\n", x.ID)

	y := builder.Init()
	fmt.Printf("Created input node y with ID: %d\n", y.ID)

	// x * y
	x_times_y := builder.Mul(x, y)
	fmt.Printf("Created (x*y) node with ID: %d\n", x_times_y.ID)

	// x / y (using hint)
	x_div_y := builder.Hint([]*graph.Node{x, y}, func(values map[graph.NodeID]uint32) uint32 {
		// Avoid division by zero
		if values[y.ID] == 0 {
			return 0
		}
		return values[x.ID] / values[y.ID]
	})
	fmt.Printf("Created (x/y) node with ID: %d\n", x_div_y.ID)

	// Constraint: (x/y) * y = x
	div_times_y := builder.Mul(x_div_y, y)
	fmt.Printf("Created ((x/y)*y) node with ID: %d\n", div_times_y.ID)

	// Add constraint: (x/y) * y = x
	builder.AssertEqual(div_times_y, x)
	fmt.Println("Added constraint: (x/y)*y = x")

	// Result: (x*y) + (x/y)
	result := builder.Add(x_times_y, x_div_y)
	fmt.Printf("Created result node (x*y)+(x/y) with ID: %d\n", result.ID)

	// Test with x = 10, y = 2
	inputs := map[graph.NodeID]uint32{
		x.ID: 10,
		y.ID: 2,
	}

	fmt.Println("\nFilling graph with x = 10, y = 2")
	values, err := builder.FillNodes(inputs)
	if err != nil {
		fmt.Printf("Error filling nodes: %v\n", err)
		return
	}

	// Print all computed values
	fmt.Println("\nComputed values:")
	fmt.Printf("x = %d\n", values[x.ID])
	fmt.Printf("y = %d\n", values[y.ID])
	fmt.Printf("x*y = %d\n", values[x_times_y.ID])
	fmt.Printf("x/y = %d\n", values[x_div_y.ID])
	fmt.Printf("(x/y)*y = %d\n", values[div_times_y.ID])
	fmt.Printf("(x*y)+(x/y) = %d\n", values[result.ID])

	// Check constraints
	fmt.Printf("\nConstraints satisfied: %v\n", builder.CheckConstraints(values))
}
