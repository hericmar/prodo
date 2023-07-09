import { RRule } from 'rrule'
import { Task } from 'stores/task-store'

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

  const now = new Date()

  if (task.completed === null) {
    // task has never been completed, check if first occurrence is in the future
    const first = rrule.after(task.created, true)
    if (first === null) {
      return RRuleEvaluation.None
    } else if (first > now) {
      return RRuleEvaluation.Future
    } else {
      return RRuleEvaluation.Missed
    }
  } else {
    const next = rrule.after(task.completed, true)
    if (next === null) {
      return RRuleEvaluation.None
    } else if (next < now) {
      return RRuleEvaluation.Missed
    } else {
      return RRuleEvaluation.Future
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
      byweekday: [RRule.MO, RRule.TU, RRule.WE, RRule.TH, RRule.FR]
    }
  }
}
