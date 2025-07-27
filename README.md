
Built by https://www.blackbox.ai

---

# PC Audio Volume Controller

## Project Overview
The **PC Audio Volume Controller** is a simple yet powerful application designed to control the volume of different audio sources on your computer using keyboard shortcuts and graphical sliders. It allows users to manage volume levels for applications like music players, web browsers, games, and system sounds efficiently.

## Installation
To get started with the PC Audio Volume Controller, follow these steps:

1. **Clone the repository:**
   ```bash
   git clone https://github.com/yourusername/pc-audio-vol-controller.git
   ```

2. **Change directory into the project:**
   ```bash
   cd pc-audio-vol-controller
   ```

3. **Install the dependencies:**
   Make sure you have [Node.js](https://nodejs.org/) installed, then run:
   ```bash
   npm install
   ```

4. **Running the app:**
   - For development mode with hot-reloading:
     ```bash
     npm run dev
     ```
   - For production mode:
     ```bash
     npm start
     ```

## Usage
Once the application is running, you can control the volume of different audio sources:

- **Keyboard Shortcuts:**
  - Use `F13 / F14` to increase/decrease the Music Player volume.
  - Use `F15 / F16` to increase/decrease the Browser volume.
  - Use `F17 / F18` to increase/decrease System Sounds volume.
  - Use `F19 / F20` to increase/decrease Game volume.
  - Use `F21` to mute/unmute all audio.
  - Use `F22` to reset all volumes to 50%.

- **Graphical Interface:**
  - The interface provides sliders for each audio source where you can adjust the volume using your mouse.

## Features
- Control multiple audio sources individually (music, browser, system, games).
- Use keyboard shortcuts for quick volume adjustments.
- Mute and unmute all audio sources with a single keypress.
- Reset all audio sources to a default volume level quickly.
- A user-friendly GUI with sliders for manual volume control.

## Dependencies
The project relies on the following packages, which will be installed automatically:
- `electron`: ^27.0.0
- `loudness`: ^0.4.2

Development dependencies include:
- `electron-reload`: ^1.5.0

## Project Structure
The project structure is organized as follows:

```
/pc-audio-vol-controller
|-- assets/                  # Directory for application assets (e.g., icons)
|-- node_modules/           # Installed npm dependencies
|-- package.json             # Project metadata and dependencies
|-- package-lock.json       # NPM lock file
|-- main.js                 # Main process for the Electron app
|-- renderer.js             # Renderer process that handles UI interactions
|-- index.html              # HTML file for the app's GUI
|-- style.css               # CSS for application styling
|-- web-version.html        # Web demo version of the application
|-- sources.json            # Configuration for audio sources and keys
```

## License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing
Contributions are welcome! If you have suggestions for improvements or new features, feel free to create an issue or submit a pull request.

## Acknowledgments
- This project utilizes Electron for building cross-platform desktop apps.
- Special thanks to the contributors and open-source community who made this project possible.

---

Feel free to reach out if you need any help or have questions!