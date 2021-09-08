const initialState = {
    message: "store.toolbar-slice.message",
  }

export default function toolbarReducer(state = initialState, action) {

    switch (action.type) {
      case 'toolbar/log': {
        let msg = state.message + 'toolbar/log'
        console.log(msg)
        alert(msg)
        return state
      }
      case 'toolbar/message/hello': {
        return {
            ...state,
            message: "store.toolbar-slice.message.hello"
        }
      }
      case 'toolbar/message/hi': {
        return {
            ...state,
            message: "store.toolbar-slice.message.hi"
        }
      }
      default:
        return state
    }
  }
