const WIDTH = 640;
const HEIGHT = 360;

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
  let controlCountGetter = wasm.instance.exports.get_control_count;
  const controlCount = controlCountGetter();
  
  let wasmMemory = wasm.instance.exports.memory;
  
  const imageDataPtr = allocateImage(WIDTH, HEIGHT);
  const keysDownPtr = allocateControls();
  const gameDataPtr = allocateGame();
  var keysDown = new Array(controlCount);
  for (var i = 0; i < controlCount; i++) {
    keysDown[i] = false;
  }
  var mouse_x = 0;
  var mouse_y = 0;
  var controls = {};
  for (var i = 0; i < controlCount; i++) {
    controls[i] = 0;
  }
  controls[0] = 87; // W
  controls[1] = 83; // S
  controls[2] = 65; // A
  controls[3] = 68; // D
  controls[4] = 27; // Escape
  controls[5] = 32; // Space
  
  addEventListener("keyup", (event) => {
    for (var i = 0; i < controlCount; i++) {
      if (event.keyCode == controls[i]) {
        keysDown[i] = false;
      }
    }
  })

  addEventListener("keydown", (event) => {
    console.log("KeyDown: ", event.keyCode);
    for (var i = 0; i < controlCount; i++) {
      if (event.keyCode == controls[i]) {
        keysDown[i] = true
      }
    }
  });
  
  addEventListener("mousemove", (event) => {
    mouse_x = event.clientX;
    mouse_y = event.clientY;
  })
  addEventListener("mousedown", (event) => {
    console.log("MouseDown: ", event.buttons)
    if (event.buttons == 1) {
      keysDown[6] = true;
    } else {
      keysDown[6] = false;
    }
  })
  addEventListener("mouseup", (event) => {
    console.log("MouseUp: ", event.buttons)
    if (event.buttons == 0) {
      keysDown[6] = false;
    }
  })
  
  const frame = (time) => {
    let delta = (time - prevTime) / 1000; // Millis to secs
    prevTime = time;
    {
      let view = new DataView(wasmMemory.buffer);
      for (var i = 0; i < controlCount; i++) {
        view.setUint8(keysDownPtr + i, keysDown[i])
      }
    }
    frameWasm(gameDataPtr, imageDataPtr, WIDTH, HEIGHT, delta, keysDownPtr, mouse_x, mouse_y);
    
    const data = new Uint8ClampedArray(wasmMemory.buffer, imageDataPtr, WIDTH * HEIGHT * 4);
    ctx.putImageData(new ImageData(data, WIDTH), 0, 0);  
    
    window.requestAnimationFrame(frame);
  };

  window.requestAnimationFrame((time) => {
    prevTime = time;
    window.requestAnimationFrame(frame);    
  });
})()
