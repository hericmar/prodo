import { defineStore } from 'pinia'
import api from 'src/api'
import { evaluateRRule, RRuleEvaluation } from 'src/utils/recurrence'
import { datetime, stripTime } from 'src/utils/datetime'

export enum TaskEvent {
  OnUpdateSave = 'task-update',
  OnUpdateCancel = 'task-update-cancel',
}

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
  dtstart?: Date | null,
  dtend?: Date | null,

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

export type FilterTaskFn = (list: any) => Task[]

export interface TaskList {
  uid: string
  name: string
  tasks: Task[]
  is_archived: boolean
  isVirtual: boolean
  onFilter: FilterTaskFn
}

export type RootState = {
  tasks: Task[],
  lists: TaskList[]
}

const FILTER_IN_LIST: FilterTaskFn = (list) => {
  return list.tasks
}

export function updateGreyedOut (task: Task) {
  task.greyedOut = task.recurrence === RRuleEvaluation.Future ||
    (task.recurrence === RRuleEvaluation.Now && task.completed !== null)
}

function toTask (task: any) {
  task.created = new Date(task.created)
  if (task.completed) {
    task.completed = new Date(task.completed)
  }
  if (task.dtstart) {
    task.dtstart = new Date(task.dtstart)
  }
  if (task.dtend) {
    task.dtend = new Date(task.dtend)
  }
  if (task.due) {
    task.due = new Date(task.due)
  }
  task.rrule = task.rrule || null

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

export const sortLists = (lists: TaskList[]) => {
  return lists.sort((a, b) => a.name.localeCompare(b.name)).sort((a, b) => {
    if (a.isVirtual) {
      return -1
    }
    if (b.isVirtual) {
      return 1
    }
    return 0
  })
}

const filterDailyTasks = (tasks: Task[], now: Date) => {
  const yesterday = new Date(now)
  yesterday.setDate(now.getDate() - 1)

  const tomorrow = new Date(now)
  tomorrow.setDate(now.getDate() + 1)

  return tasks.filter(t => {
    const hasDueSoon = t.due && datetime.isBetween(t.due, now, tomorrow)
    const isMissed = t.due && t.due < now
    const isScheduled = t.dtstart && t.dtend && datetime.isBetween(now, t.dtstart, t.dtend)

    return t.rrule ||
      hasDueSoon ||
      isMissed ||
      isScheduled
  })
}

export const useTaskStore = defineStore('task', {
  state: () => ({
    tasks: [],
    lists: []
  } as RootState),
  actions: {
    async init () {
      const tasks = await api.task.list().then(response => {
        const fetchedTasks = response.data
        fetchedTasks.forEach((task: Task) => {
          toTask(task)
        })
        return fetchedTasks
      })
      this.tasks = tasks

      const lookup = new Map<string, Task>()
      for (const task of this.tasks) {
        lookup.set(task.uid, task)
      }

      let lists: TaskList[] = await api.list.list().then(response => {
        return response.data.map((list: {uid: string, name: string, tasks: string[], is_archived: boolean}): TaskList => {
          return {
            uid: list.uid,
            name: list.name,
            // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
            tasks: list.tasks.map((taskUid: string): Task => lookup.get(taskUid)!),
            is_archived: list.is_archived,
            isVirtual: false,
            onFilter: FILTER_IN_LIST
          }
        })
      })
      lists = lists.sort((a, b) => a.name.localeCompare(b.name))

      const now = new Date()

      // Daily tasks list is a virtual list that contains all tasks with rrule or due today.
      const dailyTasksListUid = crypto.randomUUID()
      const dailyTasksList: TaskList = {
        uid: dailyTasksListUid,
        name: 'Daily Tasks',
        tasks: this.tasks,
        is_archived: false,
        isVirtual: true,
        onFilter: (list: TaskList) => {
          return filterDailyTasks(this.tasks, now)
        }
      }

      this.lists = [dailyTasksList, ...lists]
    },
    reload () {
      return this.init()
    },
    async addTask (listUid: string, payload: { summary: string }) {
      return api.task.create(listUid, payload).then(response => {
        const task = response.data
        toTask(task)
        this.tasks.splice(0, 0, task)

        const list = this.lists.find(l => l.uid === listUid)
        if (list) {
          list.tasks.splice(0, 0, task)
        }

        return this.tasks[0]
      })
    },
    async addList (payload: { name: string }) {
      return api.list.create(payload).then(response => {
        const list: TaskList = response.data
        list.onFilter = FILTER_IN_LIST
        list.isVirtual = false
        this.lists.push(list)

        return list
      })
    },
    remove (task: Task) {
      const promises = []
      const deleteFn = async (list: TaskList, task: Task) => {
        list.tasks = list.tasks.filter(t => t.uid !== task.uid)
        await api.task.delete(list.uid, task.uid)
      }
      for (const list of this.lists) {
        if (list.tasks.includes(task)) {
          promises.push(deleteFn(list, task))
        }
      }
      this.tasks = this.tasks.filter(t => t.uid !== task.uid)

      return Promise.all(promises)
    },
    removeList (uid: string) {
      api.list.delete(uid).then(() => {
        const list = this.lists.find(l => l.uid === uid)
        if (list) {
          this.tasks = this.tasks.filter(t => !list.tasks.includes(t))
          this.lists = this.lists.filter(l => l.uid !== uid)
        }
      })
    },
    update (task: Task) {
      updateGreyedOut(task)

      return api.task.update(task.uid, task).then((response) => {
        const updatedTask = response.data
        toTask(updatedTask)
        const taskIndex = this.tasks.findIndex(t => t.uid === updatedTask.uid)
        Object.assign(this.tasks[taskIndex], updatedTask)
      })
    },
    async updateList (uid: string, payload: { name?: string, is_archived?: boolean }) {
      this.lists = this.lists.map(list => {
        if (list.uid === uid) {
          list = { ...list, ...payload }
        }
        return list
      })
      this.lists = sortLists(this.lists)
      return api.list.update(uid, payload)
    },
    toggle (task: Task) {
      if (task.completed === null) {
        task.completed = new Date()
      } else {
        task.completed = null
      }
      return this.update(task)
    },
    async setOrder (listUid: string, task: Task, newIndex: number) {
      const list = this.lists.find(l => l.uid === listUid)
      if (list) {
        const oldIndex = list.tasks.indexOf(task)
        list.tasks.splice(oldIndex, 1)
        list.tasks.splice(newIndex, 0, task)

        await api.task.updatePosition(listUid, task.uid, { position: newIndex })
      }
    }
  }
})
