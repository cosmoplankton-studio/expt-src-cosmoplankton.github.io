import {wasm} from '../../index'

const initialState = {
    message: "store.global-slice.message",
  }

export default function globalReducer(state = initialState, action) {
    switch (action.type) {
      case 'global/log': {
        let msg = state.message + 'global/log'
        console.log(msg)
        alert(msg)
        return state
      }
      case 'global/pause': {
        wasm.then(m => { m.pause_update(true) })
        return state
      }
      case 'global/unpause': {
        wasm.then(m => { m.pause_update(false) })
        return state
      }
      case 'global/message/hello': {
        return {
            ...state,
            message: "store.global-slice.message.hello"
        }
      }
      case 'global/message/hi': {
        return {
            ...state,
            message: "store.global-slice.message.hi"
        }
      }
      default:
        return state
    }
  }
