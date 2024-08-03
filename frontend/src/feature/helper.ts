export const range = (begin: number, end: number) => {
  return Array.from({ length: end - begin + 1 }, (_, i) => begin + i);
}
