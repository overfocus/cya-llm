# Choose Your Own Adventure API

This project is an interactive storytelling API that generates dynamic "Choose Your Own Adventure" style narratives. It uses AI to create engaging story segments and provides users with choices to shape the direction of the story.

Brought to you by Kevin Ferron of Kevin Ferron Tech Consultancy & Digital Agency

## Features

- Dynamic story generation using AI
- Interactive storytelling with user choices
- RESTful API for easy integration
- Flexible choice handling for consistent user experience

## Getting Started

### Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)
- An API key for the AI service (Anthropic or Hugging Face, depending on your configuration)

### Installation

1. Clone the repository:
   ```
   git clone https://github.com/overfocus/cya-llm.git
   cd cya-llm
   ```

2. Set up environment variables:
   Create a `.env` file in the root directory and add your API key:
   ```
   ANTHROPIC_API_KEY=your_api_key_here
   # or
   HUGGINGFACE_API_KEY=your_api_key_here
   ```

3. Build the project:
   ```
   cargo build --release
   ```

### Running the Server

1. Start the server:
   ```
   cargo run --release
   ```

2. The server will start on `http://localhost:3000` by default.

## API Usage

The API has one main endpoint:

### Generate Story Segment

- **URL:** `/api/story/generate`
- **Method:** `POST`
- **Body:**
  ```json
  {
    "choice": 1  // Optional: The index of the user's choice (0-2)
  }
  ```
- **Success Response:**
  ```json
  {
    "story_segment": "Your generated story text goes here...",
    "choices": [
      "First choice description",
      "Second choice description",
      "Third choice description"
    ]
  }
  ```

To start a new story, send a POST request without a choice. To continue the story, include the chosen option (0-2) in subsequent requests.

## Development

### Project Structure

- `src/main.rs`: Entry point of the application
- `src/api/story.rs`: Handles the story generation API endpoint
- `src/services/llm.rs`: Interface for AI language model integration

### Adding New Features

1. Implement new endpoints in `src/api/`
2. Add new services in `src/services/`
3. Update `main.rs` to include new modules or configurations

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Thanks to [Anthropic](https://www.anthropic.com) or [Hugging Face](https://huggingface.co) for providing the AI language model used in this project.
- Inspired by the classic "Choose Your Own Adventure" books.