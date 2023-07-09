export function isTimeSet (date: Date) {
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
export function formatDate (date: Date | null, dateOnly = false) {
  if (date === null) {
    return ''
  }

  if (dateOnly) {
    return date.toISOString().substr(0, 10)
  } else {
    return date.toISOString().replace('T', ' ').substr(0, 16)
  }
}

export function stripTime (date: Date): Date {
  date.setHours(0)
  date.setMinutes(0)
  date.setSeconds(0)
  date.setMilliseconds(0)

  return date
}

export function stripSeconds (date: Date): Date {
  return new Date(date.getFullYear(), date.getMonth(), date.getDate(), date.getHours(), date.getMinutes())
}