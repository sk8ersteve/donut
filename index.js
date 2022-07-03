import { Screen, gsin, gcos } from "donut";
import { memory } from "donut/donut_bg";

const width = 600;
const height = 400;

const canvas = document.getElementById("canvas");
canvas.width = width;
canvas.height = height;
const ctx = canvas.getContext('2d');

const screen = Screen.new(width, height);
const buff = new Uint8ClampedArray(memory.buffer, screen.get_screen(), 4 * width * height);
const imgData = new ImageData(buff, width);

var A = 0.0;
var B = 0.0;

var r = 255;
var g = 0;
var b = 0;
var stage = 0;

const renderLoop = () => {
    ctx.putImageData(imgData, 0, 0);
    if (A > 6.28)
        A = 0.0;
    if (B > 6.28)
        B = 0.0;
    screen.set_donut_color(r, g, b, 255);
    screen.draw_donut(A, B);
    A += 0.01;
    B += 0.01;
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
    // screen.draw_circle();
    requestAnimationFrame(renderLoop);
}

requestAnimationFrame(renderLoop);