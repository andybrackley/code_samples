# 3D Web Game

A 3D web game built using Three.js.

## Project Structure

```text
├── index.html          # Main HTML file
├── css/
│   └── style.css      # Global styles
├── js/
│   ├── main.js        # Entry point
│   └── game.js        # Game logic and Three.js setup
```

## Running the Game

To run the game, you'll need to serve the files through a local web server. You can use any of these methods:

- Using Python: `python -m http.server`
- Using Node.js: `npx serve`

Then open your browser and navigate to `http://localhost:8000`

## Controls

- Left Arrow: Rotate the ship counterclockwise
- Right Arrow: Rotate the ship clockwise
- Up Arrow: Apply thrust in the direction the ship is facing
- Down Arrow: Apply brakes
