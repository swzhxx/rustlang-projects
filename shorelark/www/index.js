import * as sim from 'lib-simulation-wasm'

CanvasRenderingContext2D.prototype.drawTriangle = function (
  x,
  y,
  size,
  rotation
) {
  this.beginPath()

  this.moveTo(
    x - Math.sin(rotation) * size * 1.5,
    y + Math.cos(rotation) * size * 1.5
  )

  this.lineTo(
    x - Math.sin(rotation + (2.0 / 3.0) * Math.PI) * size,
    y + Math.cos(rotation + (2.0 / 3.0) * Math.PI) * size
  )

  this.lineTo(
    x - Math.sin(rotation + (4.0 / 3.0) * Math.PI) * size,
    y + Math.cos(rotation + (4.0 / 3.0) * Math.PI) * size
  )

  this.lineTo(
    x - Math.sin(rotation) * size * 1.5,
    y + Math.cos(rotation) * size * 1.5
  )

  this.stroke()
  this.fillStyle = 'rgb(255, 255, 255)' // A nice white color
  this.fill()
}

CanvasRenderingContext2D.prototype.drawCircle = function (x, y, radius) {
  this.beginPath()

  // ---
  // | Circle's center.
  // ----- v -v
  this.arc(x, y, radius, 0, 2.0 * Math.PI)
  // ------------------- ^ -^-----------^
  // | Range at which the circle starts and ends, in radians.
  // |
  // | By manipulating these two parameters you can e.g. draw
  // | only half of a circle, Pac-Man style.
  // ---

  this.fillStyle = 'rgb(0, 255, 128)' // A nice green color
  this.fill()
}
// alert("Who's that dog? " + sim.whos_that_dog() + "!");

let simulation = new sim.Simulation()

document.getElementById('train').onclick = function () {
  console.log(simulation.train())
}

const viewportScale = window.devicePixelRatio || 1
const world = simulation.world()
console.log(world)

const viewport = document.getElementById('viewport')
const ctxt = viewport.getContext('2d')
const viewportWidth = viewport.width
const viewportHeight = viewport.height
ctxt.fillStyle = 'rgb(0, 0, 0)'

function redraw() {
  ctxt.clearRect(0, 0, viewportWidth, viewportHeight)

  simulation.step()

  for (const food of world.foods) {
    ctxt.drawCircle(
      food.x * viewportWidth,
      food.y * viewportHeight,
      (0.01 / 2.0) * viewportWidth
    )
  }

  for (const animal of simulation.world().animals) {
    ctxt.drawTriangle(
      animal.x * viewportWidth,
      animal.y * viewportHeight,
      0.01 * viewportWidth,
      animal.rotation
    )
  }

  // requestAnimationFrame() schedules code only for the next frame.
  //
  // Because we want for our simulation to continue forever, we've
  // gotta keep re-scheduling our function:
  requestAnimationFrame(redraw)
}

redraw()
