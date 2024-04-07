import { Enigo, sleep } from '@enigo-js/core'

async function main() {
  const enigo = new Enigo()
  const [width, height] = enigo.mainDisplaySize()
  // console.log(width, height)
  await sleep(2000)

  // 移动鼠标到屏幕中心
  enigo.mouseMoveTo(width / 2, height / 2)
  // const [x, y] = enigo.mouseLocation()
  // console.log(x, y)
}

main()
