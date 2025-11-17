# Alphabet Game

A multilingual educational game built with Rust, Leptos, and Cloudflare Workers that helps users learn the alphabets of a language by typing words letter by letter.

## 🎮 Live Demo

Play the game at: **https://alphabet-game.david-44a.workers.dev/**

See code at: https://github.com/dhpollack/alphabet-game

## 🎯 About the Game

The Alphabet Game is an interactive educational application where players:
- Type words letter by letter from a given alphabet
- Learn the alphabets of different languages
- Practice spelling and letter recognition
- Progress through words automatically upon completion

## 🌍 Supported Languages

The game supports multiple languages including:
- English
- French
- German
- Spanish
- Russian
- Korean
- Turkish
- Arabic
- Emoji (special mode)

## 🛠️ Tech Stack

- **Frontend**: Leptos (Rust web framework)
- **Backend**: Cloudflare Workers
- **Database**: Cloudflare D1 (SQLite)
- **Styling**: Tailwind CSS
- **Deployment**: Cloudflare Workers

## 🚀 Getting Started

### Prerequisites

- Rust (latest stable)
- Node.js and npm
- Wrangler CLI (`npm install -g wrangler`)

### Development

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd alphabet-game
   ```

2. **Install dependencies**
   ```bash
   npm install
   ```

3. **Set up local database**
   ```bash
   just d1-local-migration-apply
   ```

4. **Run development server**
   ```bash
   just dev
   ```

5. **Open your browser**
   Navigate to `http://127.0.0.1:8787`

### Available Commands

- `just dev` - Start development server
- `just deploy` - Deploy to Cloudflare Workers
- `just d1-local-migration-apply` - Apply database migrations locally
- `just d1-remote-migration-apply` - Apply database migrations to production
- `just sqlx-prepare` - Prepare SQLx queries

## 📁 Project Structure

```
src/
├── app.rs              # Main application component and routing
├── components/         # UI components
│   ├── game.rs         # Main game logic
│   ├── letter_grid.rs  # Letter grid display
│   ├── header.rs       # Game header with language selector
│   └── ...
├── database.rs         # Database operations and server functions
├── game.rs             # Game state management
└── lib.rs              # Server setup and configuration
```

## 🎮 How to Play

1. **Select a language** using the dropdown in the header
2. **Look at the target word** displayed at the top
3. **Type the letters** using your keyboard or click the on-screen buttons
4. **Complete the word** to automatically advance to the next one
5. **Practice regularly** to improve your vocabulary and spelling

## 🔧 Database Schema

The game uses Cloudflare D1 with the following main tables:
- `languages` - Supported languages
- `letters` - Alphabet letters for each language
- `words` - Vocabulary words for each language

## 🚀 Deployment

The application is automatically deployed to Cloudflare Workers when changes are pushed to the main branch.

### Manual Deployment

```bash
just deploy
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit pull requests or open issues for bugs and feature requests.

### Adding New Languages

1. Add language to the `languages` table
2. Add alphabet letters to the `letters` table
3. Add vocabulary words to the `words` table

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🙏 Acknowledgments

- [Leptos](https://leptos.dev/) for the amazing Rust web framework
- [Cloudflare Workers](https://workers.cloudflare.com/) for serverless hosting
- [Tailwind CSS](https://tailwindcss.com/) for styling

---

Built with ❤️ using Rust and WebAssembly
