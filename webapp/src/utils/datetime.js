export function isTimeSet (date) {
  if (!(date instanceof Date)) {
    return false
  }

  return date.getHours() !== 0 ||
    date.getMinutes() !== 0 ||
    date.getSeconds() !== 0 ||
    date.getMilliseconds() !== 0
}

/**
 * Format date to YYYY-MM-DD format
 * @param date
 * @returns {string}
 */
export function formatDate (date) {
  if (!(date instanceof Date)) {
    return ''
  }

  return date.toISOString().substr(0, 10)
}
