package graph

import (
	"errors"
	"fmt"
)

// NodeID is a unique identifier for a node in the graph
type NodeID int

// NodeType represents the type of a node in the computational graph
type NodeType int

const (
	// InputNode is a node that requires a value to be provided
	InputNode NodeType = iota
	// ConstantNode is a node with a fixed value
	ConstantNode
	// AddNode is a node that adds two other nodes
	AddNode
	// MulNode is a node that multiplies two other nodes
	MulNode
	// HintNode is a node whose value is computed outside the graph but constrained within it
	HintNode
)

// Node represents a node in the computational graph
type Node struct {
	ID       NodeID
	Type     NodeType
	Value    uint32
	Constant uint32       // Used for ConstantNode
	Parents  [2]NodeID    // Used for AddNode and MulNode
	HintDeps []NodeID     // Used for HintNode
	HintFunc HintFunction // Used for HintNode
}

// HintFunction is a function that computes a hint value based on other node values
type HintFunction func(map[NodeID]uint32) uint32

// Constraint represents an equality constraint between two nodes
type Constraint struct {
	Left  NodeID
	Right NodeID
}

// Builder is used to create a computational graph
type Builder struct {
	Nodes       map[NodeID]*Node
	Constraints []Constraint
	NextID      NodeID
}

// NewBuilder creates a new builder
func NewBuilder() *Builder {
	return &Builder{
		Nodes:       make(map[NodeID]*Node),
		Constraints: []Constraint{},
		NextID:      0,
	}
}

// Init initializes a node in the graph
func (b *Builder) Init() *Node {
	id := b.NextID
	b.NextID++

	node := &Node{
		ID:   id,
		Type: InputNode,
	}

	b.Nodes[id] = node
	return node
}

// Constant initializes a node in the graph, set to a constant value
func (b *Builder) Constant(value uint32) *Node {
	id := b.NextID
	b.NextID++

	node := &Node{
		ID:       id,
		Type:     ConstantNode,
		Constant: value,
	}

	b.Nodes[id] = node
	return node
}

// Add adds 2 nodes in the graph, returning a new node
func (b *Builder) Add(a, c *Node) *Node {
	id := b.NextID
	b.NextID++

	node := &Node{
		ID:      id,
		Type:    AddNode,
		Parents: [2]NodeID{a.ID, c.ID},
	}

	b.Nodes[id] = node
	return node
}

// Mul multiplies 2 nodes in the graph, returning a new node
func (b *Builder) Mul(a, c *Node) *Node {
	id := b.NextID
	b.NextID++

	node := &Node{
		ID:      id,
		Type:    MulNode,
		Parents: [2]NodeID{a.ID, c.ID},
	}

	b.Nodes[id] = node
	return node
}

// AssertEqual asserts that 2 nodes are equal
func (b *Builder) AssertEqual(a, c *Node) {
	b.Constraints = append(b.Constraints, Constraint{
		Left:  a.ID,
		Right: c.ID,
	})
}

// Hint creates a node whose value is computed outside the graph but constrained within it
func (b *Builder) Hint(dependencies []*Node, computeFunc HintFunction) *Node {
	id := b.NextID
	b.NextID++

	// Extract dependency IDs
	deps := make([]NodeID, len(dependencies))
	for i, dep := range dependencies {
		deps[i] = dep.ID
	}

	node := &Node{
		ID:       id,
		Type:     HintNode,
		HintDeps: deps,
		HintFunc: computeFunc,
	}

	b.Nodes[id] = node
	return node
}

// FillNodes fills in all the nodes of the graph based on setting the values of the "input nodes"
func (b *Builder) FillNodes(inputs map[NodeID]uint32) (map[NodeID]uint32, error) {
	// Validate that all input nodes have values
	for id, node := range b.Nodes {
		if node.Type == InputNode {
			if _, ok := inputs[id]; !ok {
				return nil, fmt.Errorf("missing value for input node %d", id)
			}
		}
	}

	// Create a map to store computed values
	values := make(map[NodeID]uint32)

	// Add input values to the map
	for id, value := range inputs {
		values[id] = value
	}

	// Process nodes in order of dependencies
	for {
		progress := false

		for id, node := range b.Nodes {
			// Skip nodes that already have values
			if _, ok := values[id]; ok {
				continue
			}

			switch node.Type {
			case InputNode:
				// Already handled above
			case ConstantNode:
				values[id] = node.Constant
				progress = true
			case AddNode:
				a, aOk := values[node.Parents[0]]
				b, bOk := values[node.Parents[1]]
				if aOk && bOk {
					values[id] = a + b
					progress = true
				}
			case MulNode:
				a, aOk := values[node.Parents[0]]
				b, bOk := values[node.Parents[1]]
				if aOk && bOk {
					values[id] = a * b
					progress = true
				}
			case HintNode:
				// Check if all dependencies have values
				allDepsHaveValues := true
				depValues := make(map[NodeID]uint32)
				for _, depID := range node.HintDeps {
					if value, ok := values[depID]; ok {
						depValues[depID] = value
					} else {
						allDepsHaveValues = false
						break
					}
				}

				if allDepsHaveValues {
					// Compute the hint value
					values[id] = node.HintFunc(depValues)
					progress = true
				}
			}
		}

		// If we didn't make any progress in this iteration, we're done or stuck
		if !progress {
			break
		}
	}

	// Check if all nodes have values
	for id := range b.Nodes {
		if _, ok := values[id]; !ok {
			return nil, errors.New("unable to compute values for all nodes")
		}
	}

	return values, nil
}

// CheckConstraints checks that all the constraints hold
func (b *Builder) CheckConstraints(values map[NodeID]uint32) bool {
	for _, constraint := range b.Constraints {
		leftValue, leftOk := values[constraint.Left]
		rightValue, rightOk := values[constraint.Right]

		if !leftOk || !rightOk || leftValue != rightValue {
			return false
		}
	}

	return true
}
