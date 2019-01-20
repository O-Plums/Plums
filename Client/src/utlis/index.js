import { post, get, put, del } from 'superagent'

let serverUrl = ''

if (process.env.NODE_ENV === 'production') { serverUrl = '' }
else if (process.env.NODE_ENV === 'staging') { serverUrl = '' }
else { serverUrl = '' }

export function callApi (endpoint, method = 'get', body, psw) {

  const token = 'No token'
  return new Promise((resolve, reject) => {
    if (method === 'post') {
      post(`${serverUrl}/${endpoint}`)
        .send(body)
        .set('Accept', 'application/json')
        .set('X-Access-Token', token || '')
        .then((res) => { const body = JSON.parse(res.text); resolve(body) })
        .catch(e => {
          if (e && !e.response) { return reject('Le serveur ne répond pas. Vérifiez votre connexion Internet, Ou réessayez dans 1-2 minutes') }
          reject(e.response.body.message || e.message)
        })
    }
    if (method === 'get') {
      get(`${serverUrl}/${endpoint}`)
        .set('Accept', 'application/json')
        .set('X-Access-Token', token || '')
        .set('X-Pass', psw)
        .then((res) => { const body = JSON.parse(res.text); resolve(body) })
        .catch(e => {
          if (e && !e.response) { return reject('Le serveur ne répond pas. Vérifiez votre connexion Internet, Ou réessayez dans 1-2 minutes') }
          reject(e.response.body.message || e.message)
        })
    }

    if (method === 'put') {
      put(`${serverUrl}/${endpoint}`)
        .send(body)
        .set('Accept', 'application/json')
        .set('X-Access-Token', token || '')
        .then((res) => { const body = JSON.parse(res.text); resolve(body) })
        .catch(e => {
          if (e && !e.response) { return reject('Le serveur ne répond pas. Vérifiez votre connexion Internet, Ou réessayez dans 1-2 minutes') }
          reject(e.response.body.message || e.message)
        })
    }
    if (method === 'delete') {
      del(`${serverUrl}/${endpoint}`)
        .set('Accept', 'application/json')
        .set('X-Access-Token', token || '')
        .then((res) => { const body = JSON.parse(res.text); resolve(body) })
        .catch(e => {
          if (e && !e.response) { return reject('Le serveur ne répond pas. Vérifiez votre connexion Internet, Ou réessayez dans 1-2 minutes') }
          reject(e.response.body.message || e.message)
        })
    }
  })
}
