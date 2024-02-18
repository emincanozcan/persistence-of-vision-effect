import * as wasm from "pov";
import { memory } from "pov/pov_bg";
const { PovBoard, Pixel } = wasm;

const BOARD_WIDTH = 180;
const CELL_SIZE = Math.min(Math.floor(window.innerWidth / BOARD_WIDTH), 4);
const BOARD_HEIGHT = 135;
const BLACK_COLOR = "#000";
const WHITE_COLOR = "#FFF";
const povBoard = PovBoard.new(BOARD_WIDTH, BOARD_HEIGHT);

class CanvasManager {
  constructor() {
    this.isStopped = false;
    const canvas = document.querySelector("canvas");
    canvas.height = CELL_SIZE * BOARD_HEIGHT;
    canvas.width = CELL_SIZE * BOARD_WIDTH;
    this.canvasCtx = canvas.getContext("2d");
  }

  stop() {
    this.isStopped = true;
  }

  start() {
    this.isStopped = false;
  }

  drawCells() {
    const cellsPtr = povBoard.pixels();
    const cells = new Uint8Array(
      memory.buffer,
      cellsPtr,
      BOARD_WIDTH * BOARD_HEIGHT,
    );
    this.canvasCtx.beginPath();
    for (let row = 0; row < BOARD_HEIGHT; row++) {
      for (let col = 0; col < BOARD_WIDTH; col++) {
        const idx = row * BOARD_WIDTH + col;
        this.canvasCtx.fillStyle =
          cells[idx] === Pixel.White ? WHITE_COLOR : BLACK_COLOR;
        this.canvasCtx.fillRect(
          col * CELL_SIZE,
          row * CELL_SIZE,
          CELL_SIZE,
          CELL_SIZE,
        );
      }
    }
    this.canvasCtx.stroke();
    this.canvasCtx.closePath();
  }

  draw(...args) {
    if (this.isStopped) {
      return;
    }

    args.forEach((arg) => {
      arg.draw();
    });
    povBoard.tick();
    this.drawCells();
  }
}

class RotatingCube {
  constructor(x, y, r) {
    this.x = x;
    this.y = y;
    this.r = r;
    this.angle = 0;
  }
  draw() {
    this.angle += 0.1;
    povBoard.draw_cube(this.x, this.y, this.r, this.angle);
  }
}

class MovingLine {
  constructor(sx1, sy1, sx2, sy2, ex1, ey1, ex2, ey2) {
    this.x1 = sx1;
    this.y1 = sy1;
    this.x2 = sx2;
    this.y2 = sy2;
    this.ex1 = ex1;
    this.ey1 = ey1;
    this.ex2 = ex2;
    this.ey2 = ey2;
  }
  draw() {
    if (this.x1 < this.ex1) this.x1 += 1;
    if (this.x1 > this.ex1) this.x1 -= 1;
    if (this.y1 < this.ey1) this.y1 += 1;
    if (this.y1 > this.ey1) this.y1 -= 1;
    if (this.x2 < this.ex2) this.x2 += 1;
    if (this.x2 > this.ex2) this.x2 -= 1;
    if (this.y2 < this.ey2) this.y2 += 1;
    if (this.y2 > this.ey2) this.y2 -= 1;

    if (
      Math.abs(this.x1 - this.ex1) < 2 &&
      Math.abs(this.y1 - this.ey1) < 2 &&
      typeof this.onStop === "function"
    ) {
      return this.onStop();
    }

    povBoard.draw_line(this.x1, this.y1, this.x2, this.y2);
  }

  randomize() {
    const r = MovingLine.random();
    this.x1 = r.x1;
    this.y1 = r.y1;
    this.x2 = r.x2;
    this.y2 = r.y2;
    this.ex1 = r.ex1;
    this.ey1 = r.ey1;
    this.ex2 = r.ex2;
    this.ey2 = r.ey2;
  }

  static random() {
    const startX1 = Math.floor(Math.random() * BOARD_WIDTH);
    const startY1 = Math.floor(Math.random() * BOARD_HEIGHT);
    let startX2 = startX1 + Math.floor(Math.random() * BOARD_WIDTH);
    if (startX2 > BOARD_WIDTH) {
      startX2 = BOARD_WIDTH;
    }
    let startY2 = startY1 + Math.floor(Math.random() * BOARD_HEIGHT);
    if (startY2 > BOARD_HEIGHT) {
      startY2 = BOARD_HEIGHT;
    }

    const endX1 = Math.floor(Math.random() * BOARD_WIDTH);
    const endY1 = Math.floor(Math.random() * BOARD_HEIGHT);
    let endX2 = endX1 + Math.floor(Math.random() * BOARD_WIDTH);
    if (endX2 > BOARD_WIDTH) {
      endX2 = BOARD_WIDTH;
    }
    let endY2 = endY1 + Math.floor(Math.random() * BOARD_HEIGHT);
    if (endY2 > BOARD_HEIGHT) {
      endY2 = BOARD_HEIGHT;
    }

    return new MovingLine(
      startX1,
      startY1,
      startX2,
      startY2,
      endX1,
      endY1,
      endX2,
      endY2,
    );
  }

  setStopHandler(onStop) {
    this.onStop = onStop;
  }
}

const canvasManager = new CanvasManager();
let drawings = [];

const renderLoop = () => {
  canvasManager.draw(...drawings);
  requestAnimationFrame(renderLoop);
};
renderLoop();

let interval = null;
function demos(type) {
  if (interval) {
    clearInterval(interval);
  }
  if (type === "cube") {
    const cubeR = Math.min(BOARD_WIDTH, BOARD_HEIGHT) / 2;
    const rotatingCube = new RotatingCube(
      (BOARD_WIDTH - cubeR) / 2,
      (BOARD_HEIGHT - cubeR) / 2,
      cubeR,
    );

    drawings = [rotatingCube];
  }
  if (type === "movingLine") {
    let line = MovingLine.random();
    drawings = [line];
    interval = setInterval(() => {
      line.randomize();
    }, 250);
  }
}

const cubeButton = document.querySelector("#cube");
cubeButton.addEventListener("click", () => {
  demos("cube");
  start();
});
const lineButton = document.querySelector("#movingLine");
lineButton.addEventListener("click", () => {
  demos("movingLine");
  start();
});

const toggleStop = document.querySelector("#toggle-stop");
const start = () => {
  canvasManager.start();
  toggleStop.innerHTML = "Stop";
};
const stop = () => {
  canvasManager.stop();
  toggleStop.innerHTML = "Start";
};
toggleStop.addEventListener("click", () => {
  canvasManager.isStopped ? start() : stop();
});

demos("cube");
