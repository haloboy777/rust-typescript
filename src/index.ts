// --------------------------------------------------------------
// const foo = [1, 2, 3].map((el) => el + 1)
// console.log(foo)

// --------------------------------------------------------------
// import fs from 'fs'

// const lines = fs.readFileSync('lines') // creates a buffer

// lines
//   .toString()
//   .split('\n')
//   .filter((_, i) => i % 2 === 0)
//   .filter((_, i) => i > 1 && i < 4)
//   .forEach((el) => console.log(el))

// --------------------------------------------------------------
// enum Color {
//   Red,
//   Green,
//   Blue,
//   Yellow
// }

// function printColor(color: Color) {
//   switch (color) {
//     case Color.Red:
//       console.log('red')
//       break
//     case Color.Blue:
//       console.log('blue')
//       break
//     case Color.Green:
//       console.log('green')
//       break
//   }
// }

// printColor(Color.Yellow)

// --------------------------------------------------------------

// type Custom = {
//   name: string
//   age: number
// }

// type Item = number | string | Custom

// function append(items: Item[]) {
//   items.push('Hello fem!')
// }

// const arr: Item[] = []

// const numArr: number[] = []

// console.log(arr)
// append(arr)
// console.log(arr)

// console.log(numArr)
// append(numArr)
// console.log(numArr)

// --------------------------------------------------------------

// function practice(num: number | undefined): number {
//   return num ? num * 5 : 0
// }

// console.log(practice(10))
// console.log(practice(undefined))

// --------------------------------------------------------------

// function practice(list: number[], index: number) {
//   return (list[index] ?? index) * 5
// }

// console.log(practice([1, 2, 3], 4))

// --------------------------------------------------------------

import fs from 'fs'

const filename = process.argv[2]

if (filename) {
  const file = fs.readFileSync(filename)
  file
    .toString()
    .split('\n')
    .forEach((el) => console.log(el))
}
