# 🪙 **Coins: A Multi-Armed Bandit Simulation**

Coins is a terminal-based game built in Rust that simulates the classic multi-armed bandit problem. It allows you to explore how agents learn to make decisions through trial and error, balancing exploration and exploitation to maximize rewards. Whether you're playing as a human or training a model, Coins provides an interactive way to understand the core concepts of Reinforcement Learning.

## 🚀 How to Run Locally

To run the Coins project locally, follow these steps:

- Prerequisites:

Ensure you have Rust installed on your machine. If not, install it using the instructions on the official Rust website.

- Clone the Repository:

```bash
git clone https://github.com/your-username/coins.git
cd coins
```

- Build the Project:

```bash
cargo build --release
```

- Run the Project:

```bash
cargo run --release
```

- Explore the Menus:
  Once the application starts, you’ll be presented with a main menu where you can choose between different modes: Play, Model, and Stats.

## 🎮 Menus and Features

### 1. Play - Human Mode

In this mode, you take control and play the game yourself. You have 20 steps per episode to maximize your treasure count by choosing from 8 possible actions. Each action can result in one of the following outcomes:

- 🎰 Jackpot: +2 coins

- 💰 Treasure: +1 coin

- ❌ Bust: 0 coins

- 😞 Loss: -1 coin

- 💀 Robbed: -2 coins

Your goal is to learn which actions yield the best rewards over time. The stats screen on the right updates in real-time, showing the estimated value of each action based on your experience.

<img width="1800" alt="Screenshot 2025-02-16 at 8 47 55 PM" src="https://github.com/user-attachments/assets/e25d720d-285d-4351-b3c7-4c48327871bd" />


### 2. Model - Training Mode

This mode trains a model using an epsilon-greedy strategy to solve the multi-armed bandit problem. Here’s how it works:

With a probability of `1 - epsilon`, the model selects the action with the highest estimated value (exploitation).

With a probability of `epsilon`, it selects a random action to explore and gather more information.

As the model plays more episodes, the value of epsilon decays, meaning it explores less and exploits more. The learning rate also decays over time to stabilize the action estimates.



### 3. Stats - Training Statistics

The Stats menu provides insights into the model’s training history. You can view:

- Action Estimates: How the estimated values of each action change over time.

- Score Breakdown: A summary of rewards (Jackpots, Treasures, Losses, etc.) across episodes.

- Score Progress: The model’s performance improvement over time.

<img width="1800" alt="Screenshot 2025-02-16 at 9 14 05 PM" src="https://github.com/user-attachments/assets/6cc2bf5d-e52c-4464-9f29-28a8d166f1ad" />


## 📖 Learn More

To understand the intuition behind this project:
👉 [The Intuition Behind Value Evaluation in Reinforcement Learning](https://medium.com/@quintons831/the-intuition-behind-value-evaluation-in-rl-c14110f282b9)
