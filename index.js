import init, { Screen } from './pkg/donut.js';

// script must be loaded as module for this to work
let wasm = await init();

var width = 600;
var height = 400;
var changed = false;

const canvas = document.getElementById("canvas");
canvas.width = width;
canvas.height = height;
const ctx = canvas.getContext('2d');

var A = 0.0;
var B = 0.0;

var r = 255;
var g = 0;
var b = 0;
var stage = 0;

function set_dims(w, h) {
    width = w;
    height = h;
    canvas.width = w;
    canvas.height = h;
    changed = true
}

const screen = Screen.new(width, height);
var buff = new Uint8ClampedArray(wasm.memory.buffer, screen.get_screen(), 4 * width * height);
var imgData = new ImageData(buff, width);

screen.draw_donut(A, B, r, g, b, 255);
console.log("First donut drawn. Displaying to canvas");
ctx.putImageData(imgData, 0, 0);

const renderLoop = () => {
    // Cycle through color
    if (stage == 0) {
        r -= 1;
        g += 1;
        if (r == 0 || g == 255)
            stage = 1;
    } else if (stage == 1) {
        g -= 1;
        b += 1;
        if (g == 0 || b == 255)
            stage = 2;
    } else if (stage == 2) {
        b -= 1;
        r += 1;
        if (b == 0 || r == 255)
            stage = 0;
    }

    // Spin donut
    A += 0.04;
    B += 0.04;
    if (A > 6.28)
        A = 0.0;
    if (B > 6.28)
        B = 0.0;

    // Check if window changed
    if (changed) {
        screen.change_size(width, height);
        buff = new Uint8ClampedArray(wasm.memory.buffer, screen.get_screen(), 4 * width * height);
        imgData = new ImageData(buff, width);
        changed = false;
    }
    screen.draw_donut(A, B, r, g, b, 255);

    ctx.putImageData(imgData, 0, 0);
    requestAnimationFrame(renderLoop);
}

requestAnimationFrame(renderLoop);

