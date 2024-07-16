import { api } from 'boot/axios'
import { Task } from 'src/stores/task-store'

export default {
  auth: {
    login (username: string, password: string) {
      return api.post('/api/v1/auth/login', {
        username,
        password
      })
    },
    logout () {
      return api.post('/api/v1/auth/logout')
    },
    user () {
      return api.get('/api/v1/auth/user')
    }
  },
  list: {
    list () {
      return api.get('/api/v1/lists')
    },
    create (payload: { name: string }) {
      return api.post('/api/v1/lists', payload)
    },
    update (uid: string, payload: { name: string }) {
      return api.patch(`/api/v1/lists/${uid}`, payload)
    },
    delete (uid: string) {
      return api.delete(`/api/v1/lists/${uid}`)
    }
  },
  task: {
    list () {
      return api.get('/api/v1/tasks')
    },
    create (listUid: string, payload: { summary: string }) {
      return api.post(`/api/v1/lists/${listUid}/tasks`, payload)
    },
    update (uid: string, task: Task) {
      return api.patch(`/api/v1/tasks/${uid}`, task)
    },
    delete (listUid: string, uid: string) {
      return api.delete(`/api/v1/lists/${listUid}/tasks/${uid}`)
    },
    updatePosition (listUid: string, taskUid: string, payload: { position: number }) {
      return api.put(`/api/v1/lists/${listUid}/tasks/${taskUid}/position`, payload)
    },
    move (sourceListUid: string, targetListUid: string, taskUid: string) {
      return api.post(`/api/v1/lists/${sourceListUid}/tasks/${taskUid}/list`, { target_list_uid: targetListUid })
    }
  },
  ical: {
    get () {
      return api.get('/api/v1/calendar/subscription')
    },
    create (payload: { timezone: string }) {
      return api.post('/api/v1/calendar/subscription', payload)
    },
    update (payload: { timezone: string }) {
      return api.patch('/api/v1/calendar/subscription', payload)
    },
    delete () {
      return api.delete('/api/v1/calendar/subscription')
    }
  }
}
