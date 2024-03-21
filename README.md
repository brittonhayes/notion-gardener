# Notion Gardener 🪴

Welcome to the Notion Garden, a minimalist tool designed to help you manage your Notion to-do lists with ease. 
This Rust-based application automates the pruning of completed tasks, keeping your digital garden clean and tidy.

![Notion Gardener Logo](./.github/assets/logo.png)

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

### Configuration

1. Create a `.env` file in the root of the project directory.
2. Add the following variables:
```
PAGE_ID=<Your Notion Page ID>
API_TOKEN=<Your Notion Integration Token>
DELETE_MARKER=<Your Custom Delete Marker>
```

### 🚜 Usage

Run the application using the command: `cargo run --release`

## Environment Variables

- `PAGE_ID`: The ID of the Notion page you want to use.
- `API_TOKEN`: Your Notion integration token.
- `DELETE_MARKER`: The marker used to identify which to-dos should be deleted (e.g., "🗑️").

## 🗒️ How It Works

The Notion Gardener queries your specified Notion page for to-do items. It checks each item for two things:

1. Whether the item is marked as completed.
2. Whether the item contains the delete marker.

If it finds both of these to be true, the item will be pruned from the list, keeping your workspace tidy and focused.

## Contributing

Contributions are welcome! If you have an idea for improving this tool or have found a bug, please open an issue or submit a pull request.

## License

Distributed under the MIT License. See `LICENSE` for more information.

---

Happy gardening! 🌱
