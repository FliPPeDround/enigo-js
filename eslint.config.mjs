import antfu from '@antfu/eslint-config'

export default antfu({
  ignores: ['index.d.ts', 'index.js', 'rust-module.d.ts', 'rust-module.js'],
})
