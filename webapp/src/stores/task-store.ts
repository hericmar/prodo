import { defineStore } from 'pinia'

export interface Task {
  uid: string
  summary: string
  completed: boolean
}

export type RootState = {
  tasks: Task[]
}

export const useTaskStore = defineStore('task', {
  state: () => ({
    tasks: []
  } as RootState),
  actions: {
    addTask (summary: string) {
      const task = {
        uid: '' + this.tasks.length,
        summary,
        completed: false
      }
      this.tasks.push(task)
    }
  }
})
