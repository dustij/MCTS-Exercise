# MCTS-Exercise

### Project Description

This project is designed to help me understand and implement the Monte Carlo Tree Search (MCTS) algorithm. The focus is on applying MCTS to a simple, yet effective scenario: a game of heads or tails played over a predefined number of rounds. The primary goal is to learn how to build the tree structure, make decisions, and implement the backpropagation phase of MCTS.
Game Description

- Type: Heads or Tails Coin Flip
- Scoring: The winner of each round scores 1 point.
- Winning Condition: The game is played over a set number of rounds. The winner is determined by who has the highest score at the end.

### Implementation Details

- Monte Carlo Tree Search: The project focuses on the implementation of the MCTS algorithm, specifically:
  Tree Building: Efficiently building the tree structure representing possible game outcomes.
  Decision Making: Selecting moves based on the MCTS algorithm's exploration and exploitation balance.
  Backpropagation: Updating the tree with the outcomes of simulated games to refine decision-making in future iterations.
- Simulation: The game will play out to the predefined number of rounds during each simulation phase.
- Iterations: The MCTS will run for 'N' number of iterations within a for-loop, expanding and exploring the decision space.

### Learning Outcomes

Through this project, I aim to:

- Deepen my understanding of the Monte Carlo Tree Search algorithm.
- Gain practical experience in implementing algorithms in Rust.
- Develop skills in problem-solving and decision-making in AI.
