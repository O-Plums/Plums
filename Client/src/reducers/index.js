import { fromJS } from 'immutable'

import { login } from './user/index.js'

const intialStateUser = {
  number: '',
  name: '',
  roomName: '',
}

export default class reducer {

  static user (state = fromJS(intialStateUser), action) {
    switch (action.type) {
    case 'http/login':
      return login(state, action.data)
    default:
      return state
    }
  }

}
