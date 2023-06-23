export function isTimeSet (date) {
  if (!(date instanceof Date)) {
    return false
  }

  return date.getHours() !== 0 ||
    date.getMinutes() !== 0 ||
    date.getSeconds() !== 0 ||
    date.getMilliseconds() !== 0
}
