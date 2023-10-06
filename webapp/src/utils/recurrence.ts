import { RRule } from 'rrule'
import { Task } from 'stores/task-store'
import { stripTime } from 'src/utils/datetime'

export const BYWEEKDAY_OPTIONS = [
  {
    label: 'monday',
    value: RRule.MO.weekday
  },
  {
    label: 'tuesday',
    value: RRule.TU.weekday
  },
  {
    label: 'wednesday',
    value: RRule.WE.weekday
  },
  {
    label: 'thursday',
    value: RRule.TH.weekday
  },
  {
    label: 'friday',
    value: RRule.FR.weekday
  },
  {
    label: 'saturday',
    value: RRule.SA.weekday
  },
  {
    label: 'sunday',
    value: RRule.SU.weekday
  }
]

export enum RRuleEvaluation {
  Future,
  Missed,
  Now,
  NoMore,
  None
}

export const evaluateRRule = (task: Task) => {
  if (task.rrule === null) {
    return RRuleEvaluation.None
  }

  const rrule = RRule.fromString(task.rrule)
  if (task.start) {
    rrule.options.dtstart = task.start
  }

  const now = stripTime(new Date())

  if (task.completed === null) {
    // task has never been completed, check if first occurrence is in the future
    let first = rrule.after(task.created, true)
    console.log('first', first)
    if (first === null) {
      return RRuleEvaluation.NoMore
    }

    first = stripTime(first)
    if (first.getTime() > now.getTime()) {
      return RRuleEvaluation.Future
    } else if (first.getTime() === now.getTime()) {
      return RRuleEvaluation.Now
    } else {
      return RRuleEvaluation.Missed
    }
  } else {
    let next = rrule.after(task.completed, true)
    console.log('next', next)
    if (next === null) {
      return RRuleEvaluation.NoMore
    }

    next = stripTime(next)
    if (now.getTime() < next.getTime()) {
      return RRuleEvaluation.Future
    } else if (now.getTime() === next.getTime()) {
      return RRuleEvaluation.Now
    } else {
      return RRuleEvaluation.Missed
    }
  }
}

export default {
  daily: () => {
    return {
      freq: RRule.DAILY
    }
  },
  weekly: () => {
    return {
      freq: RRule.WEEKLY
    }
  },
  monthly: () => {
    return {
      freq: RRule.MONTHLY
    }
  },
  yearly: () => {
    return {
      freq: RRule.YEARLY
    }
  },
  each_workday: () => {
    return {
      freq: RRule.DAILY,
      byweekday: [RRule.MO, RRule.TU, RRule.WE, RRule.TH, RRule.FR]
    }
  }
}
