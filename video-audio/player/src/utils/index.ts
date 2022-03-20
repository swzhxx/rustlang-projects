export const perfomanceFn = (fn: any): any => {
  return (...args: any) => {
    const t0 = window.performance.now()
    const result = fn.apply(this, args)
    const t1 = window.performance.now()
    console.warn(`excute -----> :${t1 - t0}ms`)
    return result
  }
}
