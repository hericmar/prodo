export const datetime = {
  isSameDate (a: Date, b: Date) {
    return a.getFullYear() === b.getFullYear() &&
      a.getMonth() === b.getMonth() &&
      a.getDate() === b.getDate()
  },
  isBetween (date: Date, start: Date, end: Date) {
    return date >= start && date <= end
  }
}

export function isTimeSet (date: Date) {
  return date.getHours() !== 0 ||
    date.getMinutes() !== 0 ||
    date.getSeconds() !== 0 ||
    date.getMilliseconds() !== 0
}

export function formatDateLocal (date: Date | null | undefined, { dateOnly = false, hideYear = false } = {}) {
  if (!date) {
    return ''
  }

  if (dateOnly) {
    return date.toLocaleDateString(navigator.language, {})
  } else {
    const options: Intl.DateTimeFormatOptions = {
      month: 'numeric',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit'
    }
    if (!hideYear) {
      options.year = 'numeric'
    }

    return date.toLocaleString(navigator.language, options)
  }
}

export function formatTime (date: Date) {
  return date.toLocaleTimeString('cs-CZ', {
    hour: '2-digit',
    minute: '2-digit'
  })
}

export function stripTime (date: Date): Date {
  date.setHours(0)
  date.setMinutes(0)
  date.setSeconds(0)
  date.setMilliseconds(0)

  return date
}
