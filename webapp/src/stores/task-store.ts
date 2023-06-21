import { defineStore } from 'pinia'
import api from 'src/api'

export interface Task {
  uid: string
  summary: string
  completed?: boolean
}

export type RootState = {
  tasks: Task[],
}

export const useTaskStore = defineStore('task', {
  state: () => ({
    tasks: []
  } as RootState),
  actions: {
    init () {
      api.task.list().then(response => {
        this.tasks = response.data
      })
    },
    addTask (summary: string) {
      api.task.create(summary).then(response => {
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
    }
  }
})
