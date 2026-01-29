# Lorenzo Pulcini Portfolio

A high-performance personal portfolio website built with **Rust (Axum)**, **HTMX**, **Askama**, and **TailwindCSS**.

## ✨ Features

- **"Precision & Elegance" Design** - Modern Quantitative Finance dashboard aesthetic
- **Light/Dark Mode** - "Financial White" and "Terminal Chic" themes with smooth toggle animation
- **SPA-like Navigation** - HTMX-powered content swapping without full page reloads
- **Beautiful Animations** - Stagger fade-in, card lift effects, and smooth transitions
- **Responsive Layout** - Fixed sidebar navigation with dynamic content area
- **Type-safe Templates** - Askama compile-time checked templates

## 🛠 Tech Stack

| Component | Technology |
|-----------|------------|
| Backend | Rust + Axum |
| Frontend Interactivity | HTMX |
| Templating | Askama |
| Styling | TailwindCSS (CDN) |
| Typography | DM Serif Display + DM Sans + JetBrains Mono |

## 📁 Project Structure

```
portfolio/
├── Cargo.toml
├── src/
│   └── main.rs              # Axum router, handlers, data structures
├── templates/
│   ├── layout.html          # Base HTML shell with sidebar
│   ├── index.html           # Hero section (home)
│   ├── experience.html      # Timeline layout
│   ├── projects.html        # Projects grid
│   ├── skills.html          # Skills grouped by category
│   ├── research.html        # Publications
│   └── contact.html         # Contact information
└── static/                  # (Optional) Static assets
```

## 🚀 Quick Start

### Prerequisites

- Rust (1.70+)
- Cargo

### Installation

```bash
# Clone/navigate to project
cd portfolio

# Build the project
cargo build --release

# Run the server
cargo run --release
```

### Development

```bash
# Run with hot-reload logging
RUST_LOG=debug cargo run
```

The server will start at `http://localhost:3000`

## 🎨 Design System

### Colors

**Light Mode ("Financial White")**
- Background: `stone-50` (#fafaf9)
- Text: `slate-800` (#1e293b)
- Borders: `slate-200` (#e2e8f0)
- Accent: `gold` (#c9a227)

**Dark Mode ("Terminal Chic")**
- Background: `zinc-900` (#18181b)
- Text: `stone-100` (#f5f5f4)
- Borders: `zinc-700` (#3f3f46)
- Accent: `gold` (#c9a227)

### Animations

```css
/* Entry animation */
@keyframes fade-in-up {
    from { opacity: 0; transform: translateY(20px); }
    to { opacity: 1; transform: translateY(0); }
}

/* Stagger delays */
.delay-100 { animation-delay: 100ms; }
.delay-200 { animation-delay: 200ms; }
/* ... up to delay-900 */

/* Card hover effect */
.card-lift:hover {
    transform: translateY(-4px);
    box-shadow: 0 20px 40px -10px rgba(0, 0, 0, 0.15);
}
```

## 📍 Routes

| Route | Description |
|-------|-------------|
| `GET /` | Full page with layout |
| `GET /partials/home` | Home section (HTMX) |
| `GET /partials/experience` | Experience timeline (HTMX) |
| `GET /partials/projects` | Projects grid (HTMX) |
| `GET /partials/skills` | Skills section (HTMX) |
| `GET /partials/research` | Publications (HTMX) |
| `GET /partials/contact` | Contact info (HTMX) |

## ⌨️ Keyboard Shortcuts

- `⌘/Ctrl + K` - Toggle dark/light mode

## 📝 Content Data

All portfolio content is defined in `src/main.rs`:

- **Profile**: Name, title, tagline, summary
- **Stats**: Key metrics (years at Accenture, startups, CFA status)
- **Experience**: Timeline of work history
- **Skills**: Grouped by Quant Finance, Engineering, Certifications, Languages
- **Projects**: Startup ventures and technical projects
- **Publications**: Research papers

## 🔧 Customization

### Adding New Sections

1. Create a new template in `templates/`
2. Add a new template struct in `main.rs`
3. Add a handler function
4. Register the route in the router
5. Add navigation link in `layout.html`

### Modifying Content

Edit the `init_app_state()` function in `main.rs` to update any content.

## 📜 License

MIT License - feel free to use this as a template for your own portfolio!

---

**Built with ❤️ by Lorenzo Pulcini**

*Bridging Financial Theory and Software*
