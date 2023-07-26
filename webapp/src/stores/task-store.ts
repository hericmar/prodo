import { defineStore } from 'pinia'
import api from 'src/api'
import { evaluateRRule, RRuleEvaluation } from 'src/utils/recurrence'
import { stripTime } from 'src/utils/datetime'

export enum Urgency {
  None = 0,
  Low = 1,
  Medium = 2,
  High = 3
}

export interface Task {
  uid: string
  summary: string
  description: string
  created: Date,
  start?: Date | null,
  end?: Date | null,

  // last completed date
  completed: Date | null,
  due?: Date | null,
  rrule: string | null,

  priority: number,
  urgency: Urgency,

  // set by store when rrule is not null
  recurrence: RRuleEvaluation | null,
  greyedOut: boolean
}

export type RootState = {
  tasks: Task[],
}

function updateGreyedOut (task: Task) {
  task.greyedOut = task.recurrence === RRuleEvaluation.Future ||
    (task.recurrence === RRuleEvaluation.Now && task.completed !== null)
}

function toTask (task: any) {
  task.created = new Date(task.created)
  if (task.completed) {
    task.completed = new Date(task.completed)
  }
  if (task.start) {
    task.start = new Date(task.start)
  }
  if (task.end) {
    task.end = new Date(task.end)
  }
  if (task.due) {
    task.due = new Date(task.due)
  }

  // computed by webapp
  task.recurrence = evaluateRRule(task)
  if (task.recurrence === RRuleEvaluation.Missed) {
    // if task is missed or due today, reset last completed date
    task.completed = null
  } else if (task.recurrence === RRuleEvaluation.Now && task.completed !== null &&
    stripTime(task.completed).getTime() !== stripTime(new Date()).getTime()) {
    task.completed = null
  }

  updateGreyedOut(task)
}

export const useTaskStore = defineStore('task', {
  state: () => ({
    tasks: []
  } as RootState),
  actions: {
    async init () {
      return await api.task.list().then(response => {
        response.data.forEach((task: Task) => {
          toTask(task)
        })
        this.tasks = response.data
      })
    },
    reload () {
      this.tasks = []
      return this.init()
    },
    addTask (summary: string) {
      api.task.create(summary).then(response => {
        toTask(response.data)
        this.tasks.splice(0, 0, response.data)
      })
    },
    remove (task: Task) {
      api.task.delete(task.uid).then(() => {
        this.tasks = this.tasks.filter(t => t.uid !== task.uid)
      })
    },
    update (task: Task) {
      updateGreyedOut(task)

      return api.task.update(task.uid, task).then((response) => {
        const updatedTask = response.data
        this.tasks = this.tasks.map(t => {
          if (t.uid === updatedTask.uid) {
            toTask(updatedTask)
            return updatedTask
          }
          return t
        })
      })
    },
    toggle (task: Task) {
      if (task.completed === null) {
        task.completed = new Date()
      } else {
        task.completed = null
      }
      this.update(task)
    },
    /**
     * Indexes have to be store specific!
     * @param task
     * @param newIndex To
     */
    setOrder (task: Task, newIndex: number) {
      const oldIndex = this.tasks.indexOf(task)
      this.tasks.splice(oldIndex, 1)
      this.tasks.splice(newIndex, 0, task)

      api.task.updateOrder(task.uid, newIndex)
    }
  }
})
