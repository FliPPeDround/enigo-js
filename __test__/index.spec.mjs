import test from 'ava'

import { Coordinate, Enigo } from '../index.js'

test('move and get position', (t) => {
  const enigo = new Enigo()
  enigo.moveMouse(100, 200, Coordinate.Abs)
  const [x, y] = enigo.location()

  t.is(100, x)
  t.is(200, y)
})
