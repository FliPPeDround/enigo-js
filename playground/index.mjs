import {
  Axis,
  Button,
  Coordinate,
  Direction,
  Enigo,
  Key,
} from '@enigo-js/core'

function sleep(ms) {
  return new Promise(resolve => setTimeout(resolve, ms))
}

async function main() {
  const enigo = new Enigo()
  enigo.moveMouse(821, 845, Coordinate.Abs)
  await sleep(500)
  enigo.button(Button.Left, Direction.Press)
  await sleep(500)
  enigo.moveMouse(80, 0, Coordinate.Rel)
  await sleep(500)
  enigo.button(Button.Left, Direction.Release)
  const [x, y] = enigo.location()
  console.log({ x, y })
}

main()
