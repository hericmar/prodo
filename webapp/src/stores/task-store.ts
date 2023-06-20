import { defineStore } from 'pinia'
import api from 'src/api'

export interface Task {
  uid: string
  summary: string
  completed?: boolean
}

export type RootState = {
  tasks: Task[],
  selected: Task | null
}

export const useTaskStore = defineStore('task', {
  state: () => ({
    tasks: [],
    selected: null
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
    select (task: Task) {
      this.selected = task
    }
  }
})
