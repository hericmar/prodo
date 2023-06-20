import { api } from 'boot/axios'

export default {
  auth: {
    login (username: string, password: string) {
      return api.post('/api/v1/auth/token', {
        username,
        password
      })
    },
    logout (refreshToken: string) {
      return api.post('/api/v1/auth/token/blacklist', {
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
    delete (uid: string) {
      return api.delete(`/api/v1/tasks/${uid}`)
    }
  }
}
