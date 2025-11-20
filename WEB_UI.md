# TRM Web UI

A browser-based interface for visualizing maze navigation, training TRM models, and evaluating results - all powered by Rust and WebAssembly.

## Features

- **🎯 Maze Visualization**: Interactive canvas-based maze rendering with solution paths
- **📊 Training Panel**: Real-time training with live loss charts
- **📈 Evaluation Panel**: Model evaluation with detailed results tables
- **🦀 100% Rust**: All logic in Rust, compiled to WASM (no JavaScript!)
- **⚡ Fast**: Native performance in the browser

## Quick Start

### Prerequisites

1. Install Rust and Cargo (if not already installed)
2. Install the WASM target:
   ```bash
   rustup target add wasm32-unknown-unknown
   ```
3. Install Trunk (WASM bundler):
   ```bash
   cargo install --locked trunk
   ```

### Development Server

Start the development server with hot-reload:

```bash
./scripts/web-serve.sh
```

Or manually:

```bash
trunk serve --open
```

The UI will be available at: http://127.0.0.1:8080

### Production Build

Build optimized WASM for production:

```bash
./scripts/web-build.sh
```

Or manually:

```bash
trunk build --release
```

Output will be in the `dist/` directory.

## Features Overview

### 1. Maze Visualization

- **Interactive Canvas**: HTML5 canvas rendering of mazes
- **Solution Path**: Toggle to show/hide optimal solution
- **Dynamic Generation**: Generate new random mazes
- **Color-Coded**:
  - 🟢 Green: Start position
  - 🔴 Red: Goal position
  - ⬛ Black: Walls
  - ⬜ White: Open paths
  - 🔵 Blue: Solution path

### 2. Training Panel

**Configuration**:
- Epochs: Number of training iterations
- Learning Rate: Step size for gradient descent
- Model Architecture: Layers, H-cycles, L-cycles

**Features**:
- Start/Stop training
- Real-time loss tracking
- Live loss chart visualization
- Current epoch display
- Initial vs. current loss comparison

**Training Process**:
1. Configure hyperparameters
2. Click "Start Training"
3. Watch loss decrease in real-time
4. Stop anytime or let it complete
5. Reset to try different configurations

### 3. Evaluation Panel

**Model Info**:
- Architecture details
- Parameter count
- Dimension specifications

**Evaluation**:
- Configurable number of test examples
- Accuracy percentage
- Average loss
- Detailed results table with:
  - Input vectors
  - Target outputs
  - Model predictions
  - Correctness indicators

## Architecture

### Technology Stack

- **Yew**: Rust frontend framework (like React)
- **WebAssembly**: Compile Rust to run in browser
- **Trunk**: Build tool and dev server
- **web-sys**: Rust bindings to Web APIs
- **Canvas API**: For maze visualization

### Component Structure

```
src/web/
├── app.rs                    # Main app with tab navigation
└── components/
    ├── maze_visualizer.rs    # Maze rendering component
    ├── training_panel.rs     # Training UI
    └── evaluation_panel.rs   # Evaluation UI
```

### Data Flow

1. **Maze Visualization**:
   - Generate maze data in Rust
   - Render to Canvas using web-sys
   - User interactions trigger Rust callbacks

2. **Training**:
   - Create TRM model in Rust/WASM
   - Run training loop with gloo-timers
   - Update UI state reactively
   - Plot loss chart with SVG

3. **Evaluation**:
   - Load model in WASM
   - Generate test examples
   - Compute predictions
   - Display results in table

## Development

### File Structure

```
train-trm/
├── index.html              # Web app entry point
├── src/
│   ├── bin/
│   │   └── web.rs         # WASM entry point
│   └── web/               # Web UI components
├── scripts/
│   ├── web-serve.sh       # Dev server
│   └── web-build.sh       # Production build
└── dist/                  # Build output (generated)
```

### Building

**Development** (fast, unoptimized):
```bash
trunk serve
```

**Production** (optimized, small):
```bash
trunk build --release
```

### Configuration

Trunk configuration is in `index.html`:
```html
<link data-trunk rel="rust" data-wasm-opt="z" data-bin="web" />
```

- `data-wasm-opt="z"`: Optimize for size
- `data-bin="web"`: Use web binary

### Styling

All CSS is embedded in `index.html` for simplicity:
- Gradient backgrounds
- Responsive layout
- Tab navigation
- Form styling
- Table formatting
- Canvas styling

## Browser Compatibility

Works in all modern browsers that support WebAssembly:
- ✅ Chrome/Edge 57+
- ✅ Firefox 52+
- ✅ Safari 11+
- ✅ Opera 44+

## Performance

- **Bundle Size**: ~500KB (optimized with wasm-opt)
- **Load Time**: < 1s on modern connections
- **Training**: Real-time (10ms intervals)
- **Rendering**: 60fps canvas updates

## Debugging

Enable debug logs in browser console:
```javascript
// Already configured in web.rs via wasm-logger
```

View logs by opening browser DevTools (F12).

## Deployment

### Static Hosting

Deploy the `dist/` folder to any static host:

**GitHub Pages**:
```bash
./scripts/web-build.sh
# Copy dist/ to gh-pages branch
```

**Netlify/Vercel**:
- Build command: `trunk build --release`
- Publish directory: `dist`

**Local Server**:
```bash
cd dist
python3 -m http.server 8080
# or
npx http-server
```

## Limitations

- Training is simplified (no full backprop in demo)
- Maze generation is basic (demo purposes)
- No model persistence (page refresh resets)
- Limited to browser memory

## Future Enhancements

- [ ] Save/load models to IndexedDB
- [ ] More complex maze algorithms
- [ ] Advanced training visualizations
- [ ] 3D maze rendering with WebGL
- [ ] Multi-model comparison
- [ ] Export training data as CSV
- [ ] Real-time training metrics dashboard
- [ ] Mobile-optimized layout

## Troubleshooting

**Trunk not found**:
```bash
cargo install --locked trunk
```

**WASM target missing**:
```bash
rustup target add wasm32-unknown-unknown
```

**Build fails**:
```bash
# Clean and rebuild
cargo clean
trunk clean
trunk build --release
```

**Canvas not rendering**:
- Check browser console for errors
- Ensure Canvas API is supported
- Try refreshing the page

## Resources

- [Yew Documentation](https://yew.rs/)
- [Trunk Documentation](https://trunkrs.dev/)
- [web-sys Documentation](https://rustwasm.github.io/wasm-bindgen/api/web_sys/)
- [WebAssembly](https://webassembly.org/)

## License

Same as main project (see LICENSE file).
