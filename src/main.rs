use rand::prelude::SliceRandom;

struct GameState {
    my_score: i32,
    op_score: i32,
    my_possible_actions: Vec<Action>,
    op_possible_actions: Vec<Action>,
    round: i32,
}

#[derive(PartialEq, Clone, Copy)]
enum Action {
    Heads,
    Tails,
}

/*
Notes on Node Storage:

    Storage Without Box (e.g., Vec<Node> or Vec<Option<Node>>):
        When you store Node instances directly in a Vec<Node> or Vec<Option<Node>>, the actual
        Node data is stored inline within the vector's memory allocation. "Nodes in contiguous
        memory" means that the memory for these Node instances is allocated as a continuous block.
        When you have a vector of Node, all the Node instances are laid out sequentially in memory.
        This can be beneficial for iteration and cache locality because sequential memory access
        is typically faster. However, this also means that when the vector grows beyond its current
        capacity and needs to reallocate to a larger space, it has to copy or move these Node
        instances to the new memory location. If Node is a large struct, this can be costly in
        terms of performance.

    Storage With Box (e.g., Vec<Option<Box<Node>>>):
        Using Box<Node> means that each Node is allocated on the heap, and what the vector stores
        is a pointer (Box) to this heap-allocated Node. In this scenario, the vector itself is
        still a contiguous block of memory, but what it holds are pointers, not the actual Node
        data. The Node instances can be scattered around in different locations on the heap.
        The advantage here is that when the vector needs to grow and reallocate, only the pointers
        are copied or moved, not the entire Node structures. This is generally faster if Node is
        large. Another benefit is that heap allocation allows for dynamic sizing of each individual
        Node, which can be useful if the Node structure varies in size or if you have a very large
        number of nodes.

    In summary, using Box<Node> can make a difference in scenarios where the Node struct is large
    or when there are many nodes, as it can reduce the cost of reallocating and moving nodes in
    memory. However, it introduces an extra level of indirection, which can have a minor
    performance cost in accessing the nodes (due to pointer dereferencing). 
*/

struct Tree {
    root: Option<Box<Node>>,
}

impl Tree {
    fn new(root: Option<Box<Node>>) -> Tree {
        Tree { root }
    }
}

/*
Notes on Node Initialization:

    Expansion Phase:
        When a new node is created during the expansion phase (because it represents an unexplored
        action), this node has not yet been visited in the context of traversing the tree from the
        root to a leaf during the selection phase. Typically, in MCTS, a "visit" is counted when a
        node is encountered during this traversal. However, if you immediately play out (simulate)
        the new node after expansion, you could argue that this constitutes its first visit, as
        you're effectively exploring this new state for the first time.

    Simulation Phase:
        If you directly simulate the newly expanded node, it can be seen as the node's first visit.
        This is because the simulation phase is part of the process of evaluating and gaining
        information about this node, which is the core purpose of visiting nodes in MCTS.

    Initializing Visits to 1:
        If you choose to initialize the visit count to 1, it would mean that you are considering the
        expansion followed by an immediate simulation as the first visit to this node. This approach
        can be practical to avoid division by zero in UCB1 calculations and reflects that the node
        has already begun to contribute information to the tree. This approach requires consistency
        in how you handle visit counts throughout your algorithm. For instance, when backpropagating,
        ensure that the logic correctly interprets what these visit counts represent.

    Initializing Visits to 0:
        Alternatively, if you initialize visits to 0, it means that you consider a node as "visited"
        only when it is encountered again in a subsequent traversal from the root during the
        selection phase.
*/

struct Node {
    visits: i32,
    wins: i32,
    action: Action,
    state: GameState,
    children: Vec<Option<Box<Node>>>,
}

impl Node {
    fn new(action: Action, state: GameState) -> Node {
        Node {
            visits: 1,
            wins: 0,
            action,
            state,
            children: Vec::new(),
        }
    }

    fn contains(&self, action: &Action) -> bool {
        self.children.iter().any(|child| {
            if let Some(child) = child { &child.action == action } else { false }
        })
    }
}

impl From<Node> for Option<Box<Node>> {
    fn from(node: Node) -> Self {
        Some(Box::new(node))
    }
}

// =================================================================================================
// Monte Carlo Tree Search
// =================================================================================================

fn mcts(state: GameState, n_iterations: i32) -> Action {
    let root = Node::new(Action::Heads, state).into();
    let mut tree = Tree::new(root);

    for _ in 0..n_iterations {
        let node = select(&mut tree);

        let result = if node.state.round == 10 {
            evaluate(node)
        } else {
            let node = expand(node);
            simulate(node)
        };

        backpropagate(result);
    }

    let best_action = best_action(&tree);
    best_action
}

fn select(tree: &mut Tree) -> &mut Node {
    let mut current_node = tree.root.as_mut().unwrap();

    while is_fully_expanded(current_node) {
        current_node = best_child(current_node);
    }

    current_node.as_mut()
}

fn expand(node: &mut Node) -> &mut Box<Node> {
    let unexplored_actions = node.state.my_possible_actions
        .iter()
        .filter(|action| !node.contains(action))
        .collect::<Vec<_>>();

    // Pick a random action from the unexplored actions
    let mut rng = rand::thread_rng();
    let random_action = **unexplored_actions.choose(&mut rng).unwrap();

    let my_score = if random_action == Action::Heads {
        node.state.my_score + 1
    } else {
        node.state.my_score
    };

    let op_score = if random_action == Action::Tails {
        node.state.op_score + 1
    } else {
        node.state.op_score
    };

    // Update the game state with the random action
    let state = GameState {
        my_score,
        op_score,
        my_possible_actions: node.state.my_possible_actions.clone(),
        op_possible_actions: node.state.op_possible_actions.clone(),
        round: node.state.round + 1,
    };

    let new_node = Node::new(random_action, state).into();
    node.children.push(new_node);

    node.children.last_mut().unwrap().as_mut().unwrap()
}

fn simulate(node: &Box<Node>) -> i32 {
    unimplemented!()
}

fn backpropagate(result: i32) {
    unimplemented!()
}

fn best_action(tree: &Tree) -> Action {
    unimplemented!()
}

// ------------------------------------
// Selection Helpers
// ------------------------------------

fn is_fully_expanded(node: &mut Node) -> bool {
    unimplemented!()
}

fn best_child(node: &mut Node) -> &mut Box<Node> {
    unimplemented!()
}

fn uct_value(node: &mut Node) -> f32 {
    unimplemented!()
}

// ------------------------------------
// Simulation Helpers
// ------------------------------------

fn evaluate(node: &mut Node) -> i32 {
    unimplemented!()
}

fn main() {
    let state = GameState {
        my_score: 0,
        op_score: 0,
        my_possible_actions: vec![Action::Heads, Action::Tails],
        op_possible_actions: vec![Action::Heads, Action::Tails],
        round: 0,
    };
    mcts(state, 1000);
}
