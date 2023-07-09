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
 * Format date to YYYY-MM-DD format or YYYY-MM-DD HH:mm format
 * if dateOnly is false
 * @param date
 * @returns {string}
 */
export function formatDate (date, dateOnly = false) {
  if (!(date instanceof Date)) {
    return ''
  }

  if (dateOnly) {
    return date.toISOString().substr(0, 10)
  } else {
    return date.toISOString().replace('T', ' ').substr(0, 16)
  }
}

export function stripSeconds (date) {
  return new Date(date.getFullYear(), date.getMonth(), date.getDate(), date.getHours(), date.getMinutes())
}
