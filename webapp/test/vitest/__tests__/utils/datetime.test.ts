import { describe, expect, it } from 'vitest'
import { formatTime } from 'src/utils/datetime'

describe('formatTime', () => {
  it('formats time properly', () => {
    const date = new Date()
    date.setHours(9)
    date.setMinutes(9)

    expect(formatTime(date)).toBe('09:09')
  })
})
