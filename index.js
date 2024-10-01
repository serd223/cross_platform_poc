const WIDTH = 640;
const HEIGHT = 360;
const CONTROL_COUNT = 5;

(async () => {
  
  const wasm = await WebAssembly.instantiateStreaming(fetch("web.wasm"));
  const screenCanvas = document.getElementById("screen");
  if (screenCanvas === null) throw new Error("No canvas named 'screen' found.")
  screenCanvas.width = WIDTH;
  screenCanvas.height = HEIGHT;
  
  const ctx = screenCanvas.getContext("2d");
  if (ctx === null) throw new Error("2D context is not supported");
  ctx.imageSmoothingEnabled = false;

  let prevTime = 0;
  let frameWasm = wasm.instance.exports.frame;
  let allocateImage = wasm.instance.exports.allocate_image;
  let allocateGame = wasm.instance.exports.allocate_game;
  let allocateControls = wasm.instance.exports.allocate_controls;
  
  let wasmMemory = wasm.instance.exports.memory;
  
  const imageDataPtr = allocateImage(WIDTH, HEIGHT);
  const keysDownPtr = allocateControls();
  const gameDataPtr = allocateGame();
  var keysDown =    new Array(CONTROL_COUNT);
  const controls  = {
    0: 87, // W
    1: 83, // S
    2: 65, // A
    3: 68, // D
    4: 0
  };

  addEventListener("keyup", (event) => {
    for (var i = 0; i < CONTROL_COUNT; i++) {
      if (event.keyCode == controls[i]) {
        keysDown[i] = false;
      }
    }
  })

  addEventListener("keydown", (event) => {
    // console.log('KeyDown: ', event.keyCode);
    for (var i = 0; i < CONTROL_COUNT; i++) {
      if (event.keyCode == controls[i]) {
        keysDown[i] = true
      }
    }
  });
  
  const frame = (time) => {
    let delta = (time - prevTime) / 1000; // Millis to secs
    prevTime = time;
    {
      let view = new DataView(wasmMemory.buffer);
      for (var i = 0; i < CONTROL_COUNT; i++) {
        view.setUint8(keysDownPtr + i, keysDown[i])
      }
    }
    frameWasm(gameDataPtr, imageDataPtr, WIDTH, HEIGHT, delta, keysDownPtr);
    
    const data = new Uint8ClampedArray(wasmMemory.buffer, imageDataPtr, WIDTH * HEIGHT * 4);
    ctx.putImageData(new ImageData(data, WIDTH), 0, 0);  
    
    window.requestAnimationFrame(frame);
  };

  window.requestAnimationFrame((time) => {
    prevTime = time;
    window.requestAnimationFrame(frame);    
  });
})()
