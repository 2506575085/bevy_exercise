const fs = require('node:fs')

const buffer = fs.readFileSync('assets/json/wave_function_collapse.json')
const waveFunctionCollapse = JSON.parse(buffer)
const entries = waveFunctionCollapse.map(
  ({ code, asset_model, beside_imposible }) => {
    return [code, { code, asset_model, beside_imposible }]
  }
)

fs.writeFileSync(
  'assets/json/wave_function_collapse_map.json',
  JSON.stringify(Object.fromEntries(entries))
)
