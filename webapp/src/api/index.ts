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
    logout (refreshToken: string) {
      return api.post('/api/v1/auth/logout', {
        refresh_token: refreshToken
      })
    }
  },
  task: {
    list () {
      return api.get('/api/v1/tasks')
    },
    create (summary: string) {
      return api.post('/api/v1/tasks', {
        summary
      })
    },
    update (uid: string, task: Task) {
      return api.put(`/api/v1/tasks/${uid}`, task)
    },
    delete (uid: string) {
      return api.delete(`/api/v1/tasks/${uid}`)
    },
    updateOrder (uid: string, newIndex: number) {
      return api.put(`/api/v1/tasks/${uid}/order`, {
        order: newIndex
      })
    }
  },
  ical: {
    get () {
      return api.get('/api/v1/ical')
    },
    create () {
      return api.post('/api/v1/ical')
    },
    delete () {
      return api.delete('/api/v1/ical')
    }
  }
}
