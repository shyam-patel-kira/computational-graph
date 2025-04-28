package graph

import (
	"testing"
	"math"
)

func TestExample1(t *testing.T) {
	// Example 1: f(x) = x^2 + x + 5
	builder := NewBuilder()
	
	// Create nodes
	x := builder.Init()
	x_squared := builder.Mul(x, x)
	five := builder.Constant(5)
	x_squared_plus_x := builder.Add(x_squared, x)
	result := builder.Add(x_squared_plus_x, five)
	
	// Test with x = 3
	inputs := map[NodeID]uint32{
		x.ID: 3, // x = 3
	}
	
	values, err := builder.FillNodes(inputs)
	if err != nil {
		t.Fatalf("Failed to fill nodes: %v", err)
	}
	
	// Expected: y = 3^2 + 3 + 5 = 9 + 3 + 5 = 17
	if values[result.ID] != 17 {
		t.Errorf("Expected result to be 17, got %d", values[result.ID])
	}
	
	if !builder.CheckConstraints(values) {
		t.Errorf("Constraints check failed")
	}
}

func TestExample2(t *testing.T) {
	// Example 2: f(a) = (a+1) / 8
	builder := NewBuilder()
	
	// Create nodes
	a := builder.Init()
	one := builder.Constant(1)
	b := builder.Add(a, one)
	eight := builder.Constant(8)
	
	// Hint for division: c = b / 8
	c := builder.Hint([]*Node{b}, func(values map[NodeID]uint32) uint32 {
		return values[b.ID] / 8
	})
	
	// Constraint: c * 8 = b
	c_times_8 := builder.Mul(c, eight)
	builder.AssertEqual(c_times_8, b)
	
	// Test with a = 15
	inputs := map[NodeID]uint32{
		a.ID: 15, // a = 15
	}
	
	values, err := builder.FillNodes(inputs)
	if err != nil {
		t.Fatalf("Failed to fill nodes: %v", err)
	}
	
	// Expected: c = (15+1)/8 = 16/8 = 2
	if values[c.ID] != 2 {
		t.Errorf("Expected c to be 2, got %d", values[c.ID])
	}
	
	if !builder.CheckConstraints(values) {
		t.Errorf("Constraints check failed")
	}
}

func TestExample3(t *testing.T) {
	// Example 3: f(x) = sqrt(x+7)
	builder := NewBuilder()
	
	// Create nodes
	x := builder.Init()
	seven := builder.Constant(7)
	x_plus_seven := builder.Add(x, seven)
	
	// Hint for square root
	sqrt_x_plus_7 := builder.Hint([]*Node{x_plus_seven}, func(values map[NodeID]uint32) uint32 {
		return uint32(math.Sqrt(float64(values[x_plus_seven.ID])))
	})
	
	// Constraint: sqrt_x_plus_7 * sqrt_x_plus_7 = x_plus_seven
	computed_sq := builder.Mul(sqrt_x_plus_7, sqrt_x_plus_7)
	builder.AssertEqual(computed_sq, x_plus_seven)
	
	// Test with x = 2 (so x+7 = 9, sqrt = 3)
	inputs := map[NodeID]uint32{
		x.ID: 2,
	}
	
	values, err := builder.FillNodes(inputs)
	if err != nil {
		t.Fatalf("Failed to fill nodes: %v", err)
	}
	
	// Expected: sqrt(2+7) = sqrt(9) = 3
	if values[sqrt_x_plus_7.ID] != 3 {
		t.Errorf("Expected sqrt_x_plus_7 to be 3, got %d", values[sqrt_x_plus_7.ID])
	}
	
	if !builder.CheckConstraints(values) {
		t.Errorf("Constraints check failed")
	}
}
