import { Swarm } from "wasm-boids";
import { memory } from "wasm-boids/wasm_boids_bg.wasm";

const canvas = document.getElementById("boids-canvas");

canvas.height = window.innerHeight;
canvas.width = window.innerWidth;
const canvas_left = canvas.getBoundingClientRect().left;
const canvas_top = canvas.getBoundingClientRect().top;

const swarm = Swarm.new(canvas.width, canvas.height);

const ctx = canvas.getContext('2d');

// Interactivity:
var slider_cohesion = document.getElementById("slider-cohesion");
var slider_separation = document.getElementById("slider-separation");
var slider_alignment = document.getElementById("slider-alignment");
var slider_visual_range = document.getElementById("slider-visual-range");
var slider_mouse_interactivity = document.getElementById("slider-mouse-interactivity");
var slider_speed_limit = document.getElementById("slider-speed-limit");

slider_cohesion.oninput = function() {
  swarm.set_cohesion(this.value / (this.max - this.min));
};

slider_separation.oninput = function() {
  swarm.set_separation(this.value / (this.max - this.min));
};

slider_alignment.oninput = function() {
  swarm.set_alignment(this.value / (this.max - this.min));
};

slider_visual_range.oninput = function() {
  swarm.set_visual_range(this.value / (this.max - this.min));
};

slider_mouse_interactivity.oninput = function() {
  swarm.set_mouse_interactivity(this.value);
};

slider_speed_limit.oninput = function() {
  swarm.set_speed_limit(this.value);
}

document.onmousemove = function(e){
  swarm.set_mouse_position(e.clientX - canvas_left, e.clientY - canvas_top);
};

const renderLoop = () => {
  swarm.tick();

  drawSwarm();

  requestAnimationFrame(renderLoop);
}


const drawSwarm = () => {
  const number_boids = swarm.number_boids();
  const x_ptr = swarm.x();
  const x_vals = new Float32Array(memory.buffer, x_ptr, number_boids);
  const y_ptr = swarm.y();
  const y_vals = new Float32Array(memory.buffer, y_ptr, number_boids);
  const dx_ptr = swarm.dx();
  const dx_vals = new Float32Array(memory.buffer, dx_ptr, number_boids);
  const dy_ptr = swarm.dy();
  const dy_vals = new Float32Array(memory.buffer, dy_ptr, number_boids);

  ctx.beginPath();
  ctx.fillStyle = "#87ceeb";
  ctx.fillRect(0, 0, canvas.width, canvas.height);
  for (let k = 0; k < number_boids; ++k) {
    drawBoid(x_vals[k], y_vals[k], dx_vals[k], dy_vals[k])
    ctx.stroke();
  }

};


function drawBoid(x, y, dx, dy) {
  const angle = Math.atan2(dy, dx);
  ctx.translate(x, y);
  ctx.rotate(angle);
  ctx.translate(-x, -y);
  ctx.strokeStyle = "#000000";
  ctx.fillStyle = "#aaaaaa";
  ctx.beginPath();
  ctx.moveTo(x, y);
  ctx.lineTo(x - 20, y + 5);
  ctx.lineTo(x - 10, y);
  ctx.lineTo(x - 20, y - 5);
  ctx.lineTo(x, y);
  ctx.fill();
  ctx.setTransform(1, 0, 0, 1, 0, 0);
}

requestAnimationFrame(renderLoop);
