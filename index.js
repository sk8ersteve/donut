import init, { Screen } from './pkg/donut.js';

const width = 600;
const height = 400;

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

async function run() {
    let wasm = await init();

    const screen = Screen.new(width, height);
    console.log(screen.get_screen());
    const buff = new Uint8ClampedArray(wasm.memory.buffer, screen.get_screen(), 4 * width * height);
    const imgData = new ImageData(buff, width);

    screen.set_donut_color(r, g, b, 255);
    screen.draw_donut(A, B);
    ctx.putImageData(imgData, 0, 0);

    const renderLoop = () => {
        // Cycle through color
        if (stage == 0) {
            r -= 5;
            g += 5;
            if (r == 0 || g == 255)
                stage = 1;
        } else if (stage == 1) {
            g -= 5;
            b += 5;
            if (g == 0 || b == 255)
                stage = 2;
        } else if (stage == 2) {
            b -= 5;
            r += 5;
            if (b == 0 || r == 255)
                stage = 0;
        }
        screen.set_donut_color(r, g, b, 255);

        // Spin donut
        A += 0.05;
        B += 0.05;
        if (A > 6.28)
            A = 0.0;
        if (B > 6.28)
            B = 0.0;
        screen.draw_donut(A, B);

        ctx.putImageData(imgData, 0, 0);
        requestAnimationFrame(renderLoop);
    }

    requestAnimationFrame(renderLoop);
}

run();