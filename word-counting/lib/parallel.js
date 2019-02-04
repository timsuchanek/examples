const Parallel = require('paralleljs')

async function main() {
  const arr = [1, 2, 3]
  const p = new Parallel(arr, {
    env: {
      b: 10,
    },
  })

  p.map(function(a) {
    a * global.env.b
  })
    .reduce(([acc, curr]) => acc + curr, 0)
    .then(res => {
      console.log(res)
    })
}

main().catch(e => console.error(e))
