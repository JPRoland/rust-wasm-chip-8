import { CPU } from "wasm-chip-8";

const WIDTH = 64;
const HEIGHT = 32;

const initCanvas = (w, h) => {
  const canvas = document.getElementById("canvas");
  const ctx = canvas.getContext("2d");
  ctx.fillStyle = "rgb(16, 15, 22)";
  ctx.fillRect(0, 0, w, h);

  return ctx;
};

const updateDisplay = (state, ctx, w, h) => {
  const imageData = ctx.createImageData(w, h);

  for (let i = 0; i < state.length; i++) {
    imageData.data[i * 4] = state[i] === 1 ? 255 : 16;
    imageData.data[i * 4 + 1] = state[i] === 1 ? 95 : 15;
    imageData.data[i * 4 + 2] = state[i] === 1 ? 0 : 22;
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

const stopLoop = id => {
  window.cancelAnimationFrame(id);
};

const translateKeys = {
  "1": 0x1,
  "2": 0x2,
  "3": 0x3,
  "4": 0xc,
  q: 0x4,
  w: 0x5,
  e: 0x6,
  r: 0xd,
  a: 0x7,
  s: 0x8,
  d: 0x9,
  f: 0xe,
  z: 0xa,
  x: 0x0,
  c: 0xb,
  v: 0xf
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

// function to create new elements to append to drop down list
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
    cpu.key_down(translateKeys[e.key]);
  });

  document.addEventListener("keyup", e => {
    cpu.key_up(translateKeys[e.key]);
  });

  let runState = false;
  let animId;

  const runLoop = () => {
    for (let i = 0; i < 5; i++) {
      tick(cpu, ctx);
    }

    animId = window.requestAnimationFrame(runLoop);
  };

  const startBtn = document.getElementById("start");
  startBtn.addEventListener("click", () => {
    if (runState) {
      startBtn.innerHTML = "Start";
      stopLoop(animId);
      runState = false;
    } else {
      runState = true;
      runLoop();
      startBtn.innerHTML = "Stop";
    }
  });
})();
