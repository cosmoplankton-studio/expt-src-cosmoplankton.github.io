import { configureStore } from '@reduxjs/toolkit'

import toolbarReducer from './reducers/toolbarReducer'
import globalReducer from './reducers/globalReducer'

const store = configureStore({
  reducer: {
    toolbar: toolbarReducer,
    global: globalReducer
  }
})

export default store
