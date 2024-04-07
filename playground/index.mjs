import {
  Axis,
  Button,
  Coordinate,
  Direction,
  Enigo,
  sleep,
} from '@enigo-js/core'

async function main() {
  const enigo = new Enigo()
  enigo.button(Button.Left, Direction.Click)
  enigo.moveMouse(800, 500, Coordinate.Abs)
  enigo.button(Button.Left, Direction.Click)
  enigo.scroll(1, Axis.Vertical)
  const [x, y] = enigo.location()
  console.log({ x, y })
  const [width, height] = enigo.mainDisplay()
  console.log({ width, height })
  console.log(enigo.getDelay())
}

main()
