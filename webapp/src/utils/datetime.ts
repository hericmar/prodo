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

/**
 * Get the date format pattern for the given locale.
 * @example
 *     getDateFormatPattern('en-AU');   // dd/mm/yyyy
 *     getDateFormatPattern('en-US');   // m/d/yyyy
 */
export const getDateFormatPattern = (locale: string) => {
  const getPatternForPart = (part: Intl.DateTimeFormatPart) => {
    switch (part.type) {
      case 'day':
        return 'd'.repeat(part.value.length)
      case 'month':
        return 'M'.repeat(part.value.length)
      case 'year':
        return 'y'.repeat(part.value.length)
      case 'literal':
        return part.value
      default:
        console.log('Unsupported date part', part)
        return ''
    }
  }

  return new Intl.DateTimeFormat(locale).formatToParts(new Date('2021-01-01'))
    .map(getPatternForPart)
    .join('')
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

  const options: Intl.DateTimeFormatOptions = {
    month: 'numeric',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit'
  }
  if (!hideYear) {
    options.year = 'numeric'
  }

  if (dateOnly) {
    return date.toLocaleDateString(navigator.language, {})
  } else {
    return date.toLocaleString(navigator.language, options)
  }
}

export function formatTimeLocal (date: Date | null) {
  if (date === null) {
    return ''
  }

  return date.toLocaleTimeString(navigator.language, {
    hour: '2-digit',
    minute: '2-digit'
  })
}

/**
 * Format date using YYYY/MM/DD
 * @param date
 */
export function toDateModel (date: Date | null) : string {
  if (!date) {
    return ''
  }

  const day = date.getDate()
  const month = date.getMonth() + 1
  const year = date.getFullYear()

  return `${year}/${month < 10 ? '0' : ''}${month}/${day < 10 ? '0' : ''}${day}`
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
