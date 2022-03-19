let imageProcess: typeof import('@pkg/image-process')

import('@pkg/image-process').then((module) => {
  imageProcess = module
})

export { imageProcess }
