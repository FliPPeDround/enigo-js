import test from 'ava'

import { mouseClick, sum } from '../index.js'

test('sum from native', (t) => {
  t.is(sum(1, 2), 3)
})

test('click', () => {
  mouseClick(200, 100)
})
