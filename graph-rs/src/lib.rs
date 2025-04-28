use std::collections::{HashMap};
use std::fmt;
use std::sync::Arc;

/// A node in the computational graph.
pub struct Node {
    id: usize,
    node_type: NodeType,
}

/// The type of a node in the computational graph.
enum NodeType {
    /// An input node that requires a value to be provided.
    Input,
    /// A constant node with a fixed value.
    Constant(u32),
    /// A node that adds two other nodes.
    Add(usize, usize),
    /// A node that multiplies two other nodes.
    Mul(usize, usize),
    /// A node whose value is computed outside the graph but constrained within it.
    Hint(Vec<usize>, Arc<HintFunction>),
}

/// A wrapper for hint functions to enable cloning
struct HintFunction {
    // We use a unique ID to identify the function for debug purposes
    id: usize,
    // The actual function is stored in a Box
    func: Box<dyn Fn(&HashMap<usize, u32>) -> u32 + Send + Sync>,
}

// Implement Debug for HintFunction
impl fmt::Debug for HintFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HintFunction({})", self.id)
    }
}

impl Clone for Node {
    fn clone(&self) -> Self {
        Node {
            id: self.id,
            node_type: self.node_type.clone(),
        }
    }
}

impl Clone for NodeType {
    fn clone(&self) -> Self {
        match self {
            NodeType::Input => NodeType::Input,
            NodeType::Constant(value) => NodeType::Constant(*value),
            NodeType::Add(a, b) => NodeType::Add(*a, *b),
            NodeType::Mul(a, b) => NodeType::Mul(*a, *b),
            NodeType::Hint(deps, func) => NodeType::Hint(deps.clone(), Arc::clone(func)),
        }
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.node_type {
            NodeType::Input => write!(f, "Node({}, Input)", self.id),
            NodeType::Constant(value) => write!(f, "Node({}, Constant({}))", self.id, value),
            NodeType::Add(a, b) => write!(f, "Node({}, Add({}, {}))", self.id, a, b),
            NodeType::Mul(a, b) => write!(f, "Node({}, Mul({}, {}))", self.id, a, b),
            NodeType::Hint(deps, func) => write!(f, "Node({}, Hint({:?}, {:?}))", self.id, deps, func),
        }
    }
}

impl fmt::Debug for NodeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NodeType::Input => write!(f, "Input"),
            NodeType::Constant(value) => write!(f, "Constant({})", value),
            NodeType::Add(a, b) => write!(f, "Add({}, {})", a, b),
            NodeType::Mul(a, b) => write!(f, "Mul({}, {})", a, b),
            NodeType::Hint(deps, func) => write!(f, "Hint({:?}, {:?})", deps, func),
        }
    }
}

/// A constraint that two nodes must have equal values.
#[derive(Debug, Clone)]
struct Constraint {
    left: usize,
    right: usize,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Node({})", self.id)
    }
}

/// A builder that will be used to create a computational graph.
#[derive(Debug)]
pub struct Builder {
    nodes: Vec<Node>,
    constraints: Vec<Constraint>,
    next_id: usize,
    next_hint_id: usize,
}

impl Builder {
    /// Creates a new builder.
    pub fn new() -> Self {
        Builder {
            nodes: Vec::new(),
            constraints: Vec::new(),
            next_id: 0,
            next_hint_id: 0,
        }
    }

    /// Initializes a node in the graph.
    pub fn init(&mut self) -> Node {
        let id = self.next_id;
        self.next_id += 1;
        
        let node = Node {
            id,
            node_type: NodeType::Input,
        };
        
        self.nodes.push(node.clone());
        node
    }

    /// Initializes a node in the graph, set to a constant value.
    pub fn constant(&mut self, value: u32) -> Node {
        let id = self.next_id;
        self.next_id += 1;
        
        let node = Node {
            id,
            node_type: NodeType::Constant(value),
        };
        
        self.nodes.push(node.clone());
        node
    }

    /// Adds 2 nodes in the graph, returning a new node.
    pub fn add(&mut self, a: Node, b: Node) -> Node {
        let id = self.next_id;
        self.next_id += 1;
        
        let node = Node {
            id,
            node_type: NodeType::Add(a.id, b.id),
        };
        
        self.nodes.push(node.clone());
        node
    }

    /// Multiplies 2 nodes in the graph, returning a new node.
    pub fn mul(&mut self, a: Node, b: Node) -> Node {
        let id = self.next_id;
        self.next_id += 1;
        
        let node = Node {
            id,
            node_type: NodeType::Mul(a.id, b.id),
        };
        
        self.nodes.push(node.clone());
        node
    }

    /// Asserts that 2 nodes are equal.
    pub fn assert_equal(&mut self, a: Node, b: Node) {
        self.constraints.push(Constraint {
            left: a.id,
            right: b.id,
        });
    }

    /// An API for hinting values that allows you to perform operations
    /// like division or computing square roots.
    pub fn hint<F>(&mut self, dependencies: Vec<Node>, compute_func: F) -> Node
    where
        F: Fn(&HashMap<usize, u32>) -> u32 + Send + Sync + 'static,
    {
        let id = self.next_id;
        self.next_id += 1;
        
        let hint_id = self.next_hint_id;
        self.next_hint_id += 1;
        
        let dependency_ids = dependencies.iter().map(|node| node.id).collect();
        
        let hint_function = HintFunction {
            id: hint_id,
            func: Box::new(compute_func),
        };
        
        let node = Node {
            id,
            node_type: NodeType::Hint(dependency_ids, Arc::new(hint_function)),
        };
        
        self.nodes.push(node.clone());
        node
    }

    /// Fills in all the nodes of the graph based on setting the values of the "input nodes".
    pub fn fill_nodes(&self, inputs: HashMap<usize, u32>) -> Result<HashMap<usize, u32>, String> {
        // Debug print the nodes
        for (i, node) in self.nodes.iter().enumerate() {
            println!("Node {}: {:?}", i, node);
        }
        
        // Validate that all input nodes have values
        for node in &self.nodes {
            if let NodeType::Input = node.node_type {
                if !inputs.contains_key(&node.id) {
                    return Err(format!("Missing value for input node {}", node.id));
                }
            }
        }
        
        // Create a map to store computed values
        let mut values = HashMap::new();
        
        // Add input values to the map
        for (id, value) in inputs {
            values.insert(id, value);
        }
        
        // Process nodes in order
        for node in &self.nodes {
            match &node.node_type {
                NodeType::Input => {
                    // Already handled above
                }
                NodeType::Constant(value) => {
                    values.insert(node.id, *value);
                }
                NodeType::Add(a, b) => {
                    if let (Some(a_value), Some(b_value)) = (values.get(a), values.get(b)) {
                        values.insert(node.id, a_value.wrapping_add(*b_value));
                    } else {
                        println!("Missing values for Add operation at node {}. a={}, b={}", node.id, a, b);
                        println!("Values map: {:?}", values);
                        return Err(format!("Missing values for Add operation at node {}", node.id));
                    }
                }
                NodeType::Mul(a, b) => {
                    if let (Some(a_value), Some(b_value)) = (values.get(a), values.get(b)) {
                        values.insert(node.id, a_value.wrapping_mul(*b_value));
                    } else {
                        println!("Missing values for Mul operation at node {}. a={}, b={}", node.id, a, b);
                        println!("Values map: {:?}", values);
                        return Err(format!("Missing values for Mul operation at node {}", node.id));
                    }
                }
                NodeType::Hint(deps, func) => {
                    // Create a map of dependency values
                    let mut dep_values = HashMap::new();
                    let mut missing_deps = false;
                    
                    for &dep_id in deps {
                        if let Some(value) = values.get(&dep_id) {
                            dep_values.insert(dep_id, *value);
                        } else {
                            println!("Missing dependency value {} for Hint at node {}", dep_id, node.id);
                            missing_deps = true;
                        }
                    }
                    
                    if missing_deps {
                        return Err(format!("Missing dependency values for Hint at node {}", node.id));
                    }
                    
                    // Compute the hint value
                    let hint_value = (func.func)(&dep_values);
                    values.insert(node.id, hint_value);
                }
            }
        }
        
        Ok(values)
    }

    /// Checks that all the constraints hold.
    pub fn check_constraints(&self, values: &HashMap<usize, u32>) -> bool {
        for constraint in &self.constraints {
            let left_value = values.get(&constraint.left);
            let right_value = values.get(&constraint.right);
            
            match (left_value, right_value) {
                (Some(left), Some(right)) => {
                    if left != right {
                        return false;
                    }
                }
                _ => return false, // Missing values for constrained nodes
            }
        }
        
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        // Example 1: f(x) = x^2 + x + 5
        let mut builder = Builder::new();
        
        // Create nodes
        let x = builder.init(); // id: 0
        println!("x: {:?}", x);
        
        let x_squared = builder.mul(x.clone(), x.clone()); // id: 1
        println!("x_squared: {:?}", x_squared);
        
        let five = builder.constant(5); // id: 2
        println!("five: {:?}", five);
        
        let x_squared_plus_x = builder.add(x_squared, x); // id: 3
        println!("x_squared_plus_x: {:?}", x_squared_plus_x);
        
        let _result = builder.add(x_squared_plus_x, five); // id: 4
        println!("result: {:?}", _result);
        
        // Test with x = 3
        let mut inputs = HashMap::new();
        inputs.insert(0, 3); // x = 3
        
        let values = builder.fill_nodes(inputs).unwrap();
        println!("Final values: {:?}", values);
        
        assert_eq!(values.get(&4), Some(&17)); // y = 3^2 + 3 + 5 = 9 + 3 + 5 = 17
        assert!(builder.check_constraints(&values));
    }

    #[test]
    fn test_example_2() {
        // Example 2: f(a) = (a+1) / 8
        let mut builder = Builder::new();
        
        // Create nodes
        let a = builder.init(); // id: 0
        println!("a: {:?}", a);
        
        let one = builder.constant(1); // id: 1
        println!("one: {:?}", one);
        
        let b = builder.add(a.clone(), one); // id: 2
        println!("b: {:?}", b);
        
        let eight = builder.constant(8); // id: 3
        println!("eight: {:?}", eight);
        
        // Hint for division: c = b / 8
        let c = builder.hint(vec![b.clone()], |values| {
            println!("Hint values: {:?}", values);
            let b_value = *values.get(&2).unwrap_or(&0);
            b_value / 8
        }); // id: 4
        println!("c: {:?}", c);
        
        // Constraint: c * 8 = b
        let c_times_8 = builder.mul(c.clone(), eight); // id: 5
        println!("c_times_8: {:?}", c_times_8);
        
        builder.assert_equal(c_times_8, b);
        
        // Test with a = 15
        let mut inputs = HashMap::new();
        inputs.insert(0, 15); // a = 15
        
        let values = builder.fill_nodes(inputs).unwrap();
        println!("Final values: {:?}", values);
        
        assert_eq!(values.get(&4), Some(&2)); // c = (15+1)/8 = 16/8 = 2
        assert!(builder.check_constraints(&values));
    }

    #[test]
    fn test_example_3() {
        // Example 3: f(x) = sqrt(x+7)
        let mut builder = Builder::new();
        
        // Create nodes
        let x = builder.init(); // id: 0
        println!("x: {:?}", x);
        
        let seven = builder.constant(7); // id: 1
        println!("seven: {:?}", seven);
        
        let x_plus_seven = builder.add(x, seven); // id: 2
        println!("x_plus_seven: {:?}", x_plus_seven);
        
        // Hint for square root
        let sqrt_x_plus_7 = builder.hint(vec![x_plus_seven.clone()], |values| {
            println!("Hint values: {:?}", values);
            let x_plus_seven_value = *values.get(&2).unwrap_or(&0);
            (x_plus_seven_value as f64).sqrt() as u32
        }); // id: 3
        println!("sqrt_x_plus_7: {:?}", sqrt_x_plus_7);
        
        // Constraint: sqrt_x_plus_7 * sqrt_x_plus_7 = x_plus_seven
        let computed_sq = builder.mul(sqrt_x_plus_7.clone(), sqrt_x_plus_7.clone()); // id: 4
        println!("computed_sq: {:?}", computed_sq);
        
        builder.assert_equal(computed_sq, x_plus_seven);
        
        // Test with x = 2 (so x+7 = 9, sqrt = 3)
        let mut inputs = HashMap::new();
        inputs.insert(0, 2);
        
        let values = builder.fill_nodes(inputs).unwrap();
        println!("Final values: {:?}", values);
        
        assert_eq!(values.get(&3), Some(&3)); // sqrt(2+7) = sqrt(9) = 3
        assert!(builder.check_constraints(&values));
    }
}
