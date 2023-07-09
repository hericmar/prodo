import { defineStore } from 'pinia'
import api from 'src/api'

export interface Task {
  uid: string
  summary: string
  description: string
  created: Date,
  start?: Date | null,
  end?: Date | null,
  completed: Date | null,
  due?: Date | null,
  rrule: string | null,
}

export type RootState = {
  tasks: Task[],
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
}

export const useTaskStore = defineStore('task', {
  state: () => ({
    tasks: []
  } as RootState),
  actions: {
    init () {
      api.task.list().then(response => {
        response.data.forEach((task: Task) => {
          toTask(task)
        })
        this.tasks = response.data
      })
    },
    addTask (summary: string) {
      api.task.create(summary).then(response => {
        toTask(response.data)
        this.tasks.push(response.data)
      })
    },
    remove (task: Task) {
      api.task.delete(task.uid).then(() => {
        this.tasks = this.tasks.filter(t => t.uid !== task.uid)
      })
    },
    update (task: Task) {
      return api.task.update(task.uid, task).then(() => {
        this.tasks = this.tasks.map(t => {
          if (t.uid === task.uid) {
            return task
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
    }
  }
})
