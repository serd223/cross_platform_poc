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
  let wasmMemory = wasm.instance.exports.memory;
  const imageDataPtr = allocateImage(WIDTH, HEIGHT);

  const frame = (time) => {
    let delta = (time - prevTime) / 1000; // Millis to secs
    prevTime = time;
    
    //logic
    frameWasm(imageDataPtr, WIDTH, HEIGHT, delta);
    
    const data = new Uint8ClampedArray(wasmMemory.buffer, imageDataPtr, WIDTH * HEIGHT * 4);
    ctx.putImageData(new ImageData(data, WIDTH), 0, 0);  
    
    window.requestAnimationFrame(frame);
  };

  window.requestAnimationFrame((time) => {
    prevTime = time;
    window.requestAnimationFrame(frame);    
  });
})()
