# Notion Gardener 🪴

Notion Gardener is a minimalist tool designed to help you manage your Notion to-do lists with ease. 
This Rust-based application automates the pruning of completed tasks, keeping your digital garden clean and tidy.

<img src="./.github/assets/logo.png" width="256">

## ✨ Features

- **To-Do List Management**: Automatically removes completed tasks from your Notion to-do lists.
- **Customizable Delete Marker**: Uses a customizable delete marker for identifying tasks to delete.
- **Easy Configuration**: Configurable through environment variables for ease of use.

## 🌱 Getting Started

To start using Notion Gardener, follow these simple steps.

### Prerequisites

Ensure you have Rust installed on your machine. If you don't, follow the [official Rust installation guide](https://www.rust-lang.org/tools/install).

### Installation

1. Clone this repository to your local machine.
2. Navigate into the cloned directory.
3. Run `cargo build --release` to build the project.


### 🚜 Usage

Run the application using the command: `cargo run --release`

## Arguments and Environment Variables

- `PAGE_ID`: The ID of the Notion page you want to use.
- `API_TOKEN`: Your Notion integration token.
- `DELETE_MARKER`: The marker used to identify which to-dos should be deleted (e.g., "🗑️").

## 🗒️ How It Works

The Notion Gardener queries your specified Notion page for to-do items. It checks each item for two things:

1. The item is marked as completed
2. The item contains the delete marker

If it finds both of these to be true, the item will be pruned from the list, keeping your workspace tidy and focused.

![image](https://github.com/brittonhayes/notion-gardener/assets/46035482/be689632-4cd7-4f13-9fa3-2d40ce3025b3)

## Contributing

Contributions are welcome! If you have an idea for improving this tool or have found a bug, please open an issue or submit a pull request.

## License

Distributed under the MIT License. See `LICENSE` for more information.

---

Happy gardening! 🌱
