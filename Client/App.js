import React, { Component } from 'react'
import { createStore, applyMiddleware, combineReducers } from 'redux'
import { Provider } from 'react-redux'
// TODO => middleware can be relly good for WS
// import simpleMiddleWare from './src/middleware/index.js'
import thunk from 'redux-thunk'
import reducer from './src/reducers'

import App from './src/app.js'

const configureStore = (reducer) => createStore(
  combineReducers({
    user: reducer.user,
  }),
  applyMiddleware(
   // simpleMiddleWare(),
   thunk
 ),

)

const store =  configureStore(reducer)

class Plums extends Component {

  render() {
    return (
      <Provider store={store}>
        <App />
      </Provider>
    );
  }
}
export default Plums
