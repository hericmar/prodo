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
    update (uid: string, payload: { name: string }) {
      return api.patch(`/api/v1/lists/${uid}`, payload)
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
    delete (uid: string) {
      return api.delete(`/api/v1/tasks/${uid}`)
    },
    updatePosition (listUid: string, taskUid: string, payload: { position: number }) {
      return api.put(`/api/v1/lists/${listUid}/tasks/${taskUid}/position`, payload)
    }
  },
  ical: {
    get () {
      return api.get('/api/v1/calendar/subscription')
    },
    create () {
      return api.post('/api/v1/calendar/subscription')
    },
    delete () {
      return api.delete('/api/v1/calendar/subscription')
    }
  }
}
