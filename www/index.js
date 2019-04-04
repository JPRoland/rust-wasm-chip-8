import { CPU } from "wasm-chip-8";

const WIDTH = 64;
const HEIGHT = 32;

const initCanvas = (w, h) => {
  const canvas = document.getElementById("canvas");
  const ctx = canvas.getContext("2d");
  ctx.fillStyle = "black";
  ctx.fillRect(0, 0, w, h);

  return ctx;
};

const updateDisplay = (state, ctx, w, h) => {
  const imageData = ctx.createImageData(w, h);

  for (let i = 0; i < state.length; i++) {
    imageData.data[i * 4] = state[i] === 1 ? 0x33 : 0;
    imageData.data[i * 4 + 1] = state[i] === 1 ? 0xff : 0;
    imageData.data[i * 4 + 2] = state[i] === 1 ? 0x66 : 0;
    imageData.data[i * 4 + 3] = 255;
  }

  ctx.putImageData(imageData, 0, 0);
};

const getGameData = async rom => {
  const res = await window.fetch(`roms/${rom}`);
  const gameData = await res.arrayBuffer();

  return new Uint8Array(gameData);
};

const tick = (cpu, ctx) => {
  const cpuOutput = cpu.emulate_cycle();
  let displayState = cpuOutput.get_vram();

  updateDisplay(displayState, ctx, WIDTH, HEIGHT);
};

const runLoop = (cpu, ctx) => {
  return window.setInterval(tick.bind(this, cpu, ctx), 2);
};

const stopLoop = interval => {
  window.clearInterval(interval);
};

const translateKeys = {
  49: 0x1, // 1
  50: 0x2, // 2
  51: 0x3, // 3
  52: 0xc, // 4
  81: 0x4, // Q
  87: 0x5, // W
  69: 0x6, // E
  82: 0xd, // R
  65: 0x7, // A
  83: 0x8, // S
  68: 0x9, // D
  70: 0xe, // F
  90: 0xa, // Z
  88: 0x0, // X
  67: 0xb, // C
  86: 0xf // V
};

const ROMS = [
  "15PUZZLE",
  "BLINKY",
  "BLITZ",
  "BRIX",
  "CONNECT4",
  "GUESS",
  "HIDDEN",
  "INVADERS",
  "KALEID",
  "MAZE",
  "MERLIN",
  "MISSILE",
  "PONG",
  "PONG2",
  "PUZZLE",
  "SYZYGY",
  "TANK",
  "TETRIS",
  "TICTAC",
  "UFO",
  "VBRIX",
  "VERS",
  "WIPEOFF"
];

// Create new elements to append to drop down list
const el = domStr => {
  const html = new DOMParser().parseFromString(domStr, "text/html");

  return html.body.firstChild;
};

window.onload = (async () => {
  const ctx = initCanvas(WIDTH, HEIGHT);
  const cpu = CPU.new();

  let roms = document.getElementById("roms");
  ROMS.forEach(rom => {
    roms.append(el(`<option value="${rom}">${rom}</option>`));
  });

  roms.addEventListener("change", async e => {
    const gameData = await getGameData(e.target.value);
    cpu.reset();
    cpu.load(gameData);
  });

  document.addEventListener("keydown", e => {
    cpu.key_down(translateKeys[e.keyCode]);
  });

  document.addEventListener("keyup", e => {
    cpu.key_up(translateKeys[e.keyCode]);
  });

  let runState = false;
  let interval;

  const startBtn = document.getElementById("start");
  startBtn.addEventListener("click", () => {
    if (runState) {
      startBtn.innerHTML = "Start";
      stopLoop(interval);
      runState = false;
    } else {
      runState = true;
      interval = runLoop(cpu, ctx);
      startBtn.innerHTML = "Stop";
    }
  });
})();
