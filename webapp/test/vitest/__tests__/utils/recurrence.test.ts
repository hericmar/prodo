import { RRule } from 'rrule'
import { evaluateRRule, RRuleEvaluation } from 'src/utils/recurrence'
import { describe, expect, it } from 'vitest'
import { Task } from 'stores/task-store'

describe('Recurrence evaluation', () => {
  it('', () => {
    const now = new Date()
    const year = now.getFullYear()
    const month = now.getMonth()
    const day = now.getDate()
    const hour = now.getHours()
    const minute = now.getMinutes()

    const data = [
      {
        // future occurrence
        datetime: Date.UTC(year, month, day, hour, minute),
        options: {
          freq: RRule.MONTHLY,
          dtstart: new Date(Date.UTC(year + 1, month, day, hour, minute)),
          count: 4
        },
        created: new Date(),
        completed: null,
        result: RRuleEvaluation.Future
      },
      {
        // missed occurrence
        options: {
          freq: RRule.MONTHLY,
          dtstart: new Date(Date.UTC(year - 1, month, day, hour, minute)),
          count: 128
        },
        created: new Date(Date.UTC(year - 1, month, day, hour, minute)),
        completed: new Date(Date.UTC(year - 1, month + 2, day, hour, minute)),
        result: RRuleEvaluation.Missed
      }
    ]

    for (let i = 0; i < data.length; i++) {
      const item = data[i]
      const rrule = new RRule(item.options)
      const task = {
        created: item.created,
        completed: item.completed,
        rrule: rrule.toString()
      }
      expect(evaluateRRule(task as Task)).toBe(item.result)
    }

    expect(true).toBeTruthy()
  })
})
