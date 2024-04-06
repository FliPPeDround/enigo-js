import { Enigo } from '../index.js'

const enigo = new Enigo()
enigo.mouseMoveTo(0,0)
enigo.mouseClick('right')
const [x,y] = enigo.mouseLocation()
console.log(x,y)
