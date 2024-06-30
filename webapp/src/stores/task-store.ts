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

  lists: Set<string>
}

export type FilterTaskFn = (uid: string, tasks: Task[]) => Task[]

export interface TaskList {
  uid: string
  name: string
  isVirtual: boolean
  onFilter: FilterTaskFn
  onTaskCreate: () => void
}

export type RootState = {
  tasks: Task[],
  lists: TaskList[],
  list: {
    uid: string,
    name: string
  }
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

  task.lists = new Set()
}

export const useTaskStore = defineStore('task', {
  state: () => ({
    tasks: [],
    lists: [],
    list: {
      uid: '',
      name: ''
    }
  } as RootState),
  actions: {
    async init () {
      const defaultOnFilter = (uid: string, tasks: Task[]) => {
        return tasks.filter(t => t.lists.has(uid))
      }
      /*
      const defaultOnTaskCreate = (uid: string, task: Task) => {
        this.addTask(uid, { summary: '' })
      }
       */

      const { data } = await api.list.list()
      const fetchedLists: { uid: string, tasks: string[] }[] = data
      const lists = data.map((list: Partial<TaskList>) => {
        return {
          uid: list.uid,
          name: list.name,
          isVirtual: false,
          onFilter: defaultOnFilter,
          onTaskCreate: () => {
            this.addTask(list.uid || '', { summary: '' })
          }
        }
      })
      this.lists = lists.sort((a: TaskList, b: TaskList) => a.name.localeCompare(b.name))

      const dailyTasksListUid = crypto.randomUUID()
      const dailyTasksList = {
        uid: dailyTasksListUid,
        name: 'Daily Tasks',
        isVirtual: true,
        onFilter: (uid: string, tasks: Task[]) => {
          return tasks.filter(t => t.rrule)
        },
        onTaskCreate: () => {
          // TODO: add task with rrule daily
          // this.addTask(dailyTasksListUid, { summary: '' })
        }
      }
      this.lists = [dailyTasksList, ...this.lists]

      await api.task.list().then(response => {
        response.data.forEach((task: Task) => {
          toTask(task)
        })
        this.tasks = response.data
      })

      for (const task of this.tasks) {
        for (const list of fetchedLists) {
          for (const taskWithinListUid of list.tasks) {
            if (taskWithinListUid === task.uid) {
              task.lists.add(list.uid)
            }
          }
        }
      }
    },
    reload () {
      this.tasks = []
      return this.init()
    },
    async addTask (uid: string, payload: { summary: string }) {
      return api.task.create(uid, payload).then(response => {
        const task = response.data
        toTask(task)
        this.tasks.splice(0, 0, task)

        const list = this.lists.find(l => l.uid === uid)
        if (list) {
          task.lists.add(uid)
        }

        return this.tasks[0]
      })
    },
    async addList (payload: { name: string }) {
      return api.list.create(payload).then(response => {
        const list = response.data
        const defaultOnFilter = (uid: string, tasks: Task[]) => {
          return tasks.filter(t => t.lists.has(uid))
        }
        list.onFilter = defaultOnFilter
        list.isVirtual = false
        this.lists.push(list)

        return list
      })
    },
    remove (task: Task) {
      const promises = []
      for (const listUid of task.lists) {
        promises.push(api.task.delete(listUid, task.uid).then(() => {
          this.tasks = this.tasks.filter(t => t.uid !== task.uid)
        }))
      }
      return Promise.all(promises)
    },
    removeList (uid: string) {
      api.list.delete(uid).then(() => {
        this.tasks = this.tasks.filter(t => !t.lists.has(uid))
        this.lists = this.lists.filter(l => l.uid !== uid)
      })
    },
    update (task: Task) {
      updateGreyedOut(task)

      return api.task.update(task.uid, task).then((response) => {
        const updatedTask = response.data
        const taskLists = task.lists
        this.tasks = this.tasks.map(t => {
          if (t.uid === updatedTask.uid) {
            toTask(updatedTask)
            updatedTask.lists = taskLists

            return updatedTask
          }
          return t
        })
      })
    },
    async updateList (uid: string, payload: { name: string }) {
      this.lists = this.lists.map(list => {
        if (list.uid === uid) {
          list.name = payload.name
        }
        return list
      })
      return api.list.update(uid, payload)
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

      return api.task.updatePosition(this.list.uid, task.uid, { position: newIndex })
    }
  }
})
