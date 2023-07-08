import { RRule } from 'rrule'

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
