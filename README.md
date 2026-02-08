# Nintendo Switch - Web UI & Tauri Game Launcher
A remake of the Nintendo Switch UI in React and Tailwind. This project was originally a fun practice exercise but has been enhanced as a Tauri desktop application that can scan and launch Nintendo Switch games from Ryujinx and Yuzu emulators.

![Screenshot of the Nintendo Switch UI](https://i.imgur.com/9uPoUGf.png)

## Features

### Web Version
- Rotate your phone and double tap to go into full-screen mode
- Fully touch-compatible gallery (swipe through it!)
- Click on a game tile to select, and use arrow keys to move
- Uses the actual time and battery percent

### Tauri Desktop App (NEW!)
- **Automatic game scanning** from Ryujinx and Yuzu emulator directories
- **Game icon extraction** with multiple strategies:
  - Extracts title IDs from game filenames
  - Fetches icons from tinfoil.media
  - Scans for local icon files
  - Checks Ryujinx cache for game icons
- **Launch games directly** from the UI with one click (A button) or Enter key
- **Native desktop experience** with the Nintendo Switch UI aesthetic

## Try it out!

### Web Version
Go to [kirankunigiri.com/nintendo-switch-web-ui](https://kirankunigiri.com/nintendo-switch-web-ui) to see it in action. If you're on a phone, you'll want to double-tap to open it in full screen and rotate your device. Then swipe left and right to move around the game carousel. You might even be able to trick people into thinking you're emulating the Switch OS on your phone!

If you're on PC, try clicking on a game tile and using your arrow keys or scroll wheel to move around.

### Tauri Desktop App
Build and run the desktop application to automatically scan and launch your Nintendo Switch games from Ryujinx or Yuzu emulators. The app will:
1. Automatically scan standard Ryujinx and Yuzu directories
2. Extract game information and icons
3. Display them in the beautiful Switch UI
4. Launch games with a single click or by pressing Enter

---

### Todos
These are some cool things planned for future development:
- [x] Tauri desktop app integration
- [x] Automatic game scanning from Ryujinx
- [x] Automatic game scanning from Yuzu
- [x] Game icon extraction and display
- [x] Launch games from the UI
- [ ] Update battery icon and color to match real status
- [ ] Allow proper movement with arrow keys (it currently will not move the selected tile on the edges of the carousel)
- [ ] Add Options and Start button functionality
- [ ] Dark/light theme
- [ ] Manual game addition UI
- [ ] Game settings and favorites
- [ ] Custom game icons support

---

### Development

#### Web Version
Just run npm and get started!
```bash
npm install --legacy-peer-deps
npm run dev
```

#### Tauri Desktop App
To run the Tauri desktop application:

**Prerequisites:**
- Node.js and npm
- Rust and Cargo ([Install from rustup.rs](https://rustup.rs/))
- System dependencies:
  - **Linux**: `sudo apt install libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf libgtk-3-dev`
  - **macOS**: No additional dependencies needed (uses system WebKit)
  - **Windows**: No additional dependencies needed (uses WebView2)

**Run in development mode:**
```bash
npm install --legacy-peer-deps
npm run tauri:dev
```

**Build for production:**
```bash
npm run tauri:build
```

The built application will be in `src-tauri/target/release/`.

**Game Directory Scanning:**
The app automatically scans these directories:
- **Ryujinx**:
  - Linux: `~/.config/Ryujinx/games`
  - macOS: `~/Library/Application Support/Ryujinx/games`
  - Windows: `%APPDATA%/Roaming/Ryujinx/games`
- **Yuzu**:
  - Linux: `~/.local/share/yuzu/load`
  - macOS: `~/Library/Application Support/yuzu/load`
  - Windows: `%APPDATA%/Roaming/yuzu/load`
- **Additional directories scanned** (up to 3 levels deep):
  - `~/Documents/Yuzu/games`
  - `~/Games/Switch`
  - `~/Games/Yuzu`
  - `~/Downloads/Switch`
  - `~/Downloads` (for games stored directly in Downloads folder)

Place your `.nsp`, `.xci`, `.nca`, or `.nro` game files in these directories for automatic detection.

**Tools**
- In App.tsx, you can change from the `Console` component to the `Slider` or `Fade` component for assisted UI development
- `Slider` - this component gives you a slider to drag to compare the `Console` component with a screenshot of the actual Switch UI
- `Fade` - this component directly overlays the `Console` component over a UI screenshot with an opacity fade to see both at the same time

---

### Layout
Why does this project use `em` instead of `rem` or `px`? In this project, I wanted to create an exact replica of the UI, and not focus too much on making things responsive. I scaled the `Console` component to be exactly 16:9 so I could focus on actually building the UI, and have it scale exactly along with the root 16:9 div. You'll notice that it is technically still "responsive" as the game tile carousel stretches to fit any width. That was a fix I added so it would look like a console on mobile devices.

It would have been even better if the items on the left and right scaled nicely, and there are lots of room for improvement here.


