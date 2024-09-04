import { World, Direction, GameStatus } from "snake_game";
import { memory } from "snake_game/snake_game_bg.wasm";
import { rnd } from "./utils/random";

function init() {
  const CELL_SIZE = 30;
  const WORLD_WIDTH = 8;
  const snakeSpawnIdx = rnd(WORLD_WIDTH * WORLD_WIDTH);

  const world = World.new(WORLD_WIDTH, snakeSpawnIdx);
  const worldWidht = world.get_width();

  const points = document.getElementById("points");
  const canvas = <HTMLCanvasElement>document.getElementById("snake-canvas");
  const gameStatus = document.getElementById("game-status");
  const gameControlBtn = document.getElementById("game-control-btn");
  const ctx = canvas.getContext("2d") as CanvasRenderingContext2D;

  canvas.height = worldWidht * CELL_SIZE;
  canvas.width = worldWidht * CELL_SIZE;

  gameControlBtn.addEventListener("click", (_) => {
    const status = world.get_game_status();

    if (!status) {
      gameControlBtn.textContent = "Playing...";
      world.start_game();
      play();
    } else {
      location.reload();
    }
  });

  // const snakeCellPtr = world.snake_cell_ptr();
  // const snakeLen = world.get_snake_length();

  // const snakeCells = new Uint32Array(wasmMemory.buffer, snakeCellPtr, snakeLen);
  // console.log("snakeCells", snakeCells);

  type VoidFunction = () => void;
  const directions: Record<string, VoidFunction> = {
    ArrowUp: () => world.change_snake_dir(Direction.Up),
    ArrowDown: () => world.change_snake_dir(Direction.Down),
    ArrowLeft: () => world.change_snake_dir(Direction.Left),
    ArrowRight: () => world.change_snake_dir(Direction.Right),
  };

  document.addEventListener("keydown", (e) => {
    const direction = directions[e.code];
    if (direction) direction();
    // You can use the direction value as needed
  });

  function drawWorld() {
    ctx.beginPath();

    // Drawing vertical lines
    for (let x = 0; x <= worldWidht; x++) {
      ctx.moveTo(CELL_SIZE * x, 0);
      ctx.lineTo(CELL_SIZE * x, worldWidht * CELL_SIZE);
    }

    // Drawing horizontal lines
    for (let y = 0; y <= worldWidht; y++) {
      ctx.moveTo(0, CELL_SIZE * y);
      ctx.lineTo(worldWidht * CELL_SIZE, CELL_SIZE * y);
    }

    ctx.stroke();
  }

  function drawApple() {
    const idx = world.get_apple();
    // console.log("apple index is:", idx);
    const col = idx % worldWidht;
    const row = Math.floor(idx / worldWidht);
    // console.log(`apple ar row ${row + 1} and col ${col + 1}`);
    ctx.beginPath();
    ctx.fillStyle = "#FF0000";
    ctx.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE);
    ctx.stroke();
  }

  function drawSnake() {
    const snakeCells = new Uint32Array(
      memory.buffer,
      world.snake_cell_ptr(),
      world.get_snake_length()
    );

    // console.log("Snake cell", snakeCells);
    // debugger;

    snakeCells
      .filter((cellIdx, idx) => !(idx > 0 && cellIdx === snakeCells[0]))
      .forEach((cellIdx, idx) => {
        const col = cellIdx % worldWidht; // gets reminder of division to get the column number independently of the row
        const row = Math.floor(cellIdx / worldWidht);

        ctx.fillStyle = idx === 0 ? "#7878db" : "#000000";

        ctx.beginPath();
        ctx.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE);

        ctx.stroke();
      });
  }

  function drawGameStatus() {
    gameStatus.textContent = world.get_game_status_text();
    points.textContent = world.get_points().toString();
  }

  function paint() {
    drawWorld();
    drawSnake();
    drawApple();
    drawGameStatus();
  }

  function play() {
    const status = world.get_game_status();

    if (status === GameStatus.Lost || status === GameStatus.Won) {
      gameControlBtn.textContent = "Re-play";
      return;
    }

    const fps = 4;
    setTimeout(() => {
      ctx.clearRect(0, 0, canvas.width, canvas.height);
      world.next_position();
      paint();
      requestAnimationFrame(play);
    }, 1000 / fps);
  }
  // debugger;
  paint();
}

init();
