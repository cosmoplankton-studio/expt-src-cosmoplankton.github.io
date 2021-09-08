import webApp from '../../app'

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
        webApp.pauseUpdate(true)
        return state
      }
      case 'global/unpause': {
        webApp.pauseUpdate(false)
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
