import { configureStore } from '@reduxjs/toolkit'
import { pictureOfTheDayApi } from '../api/pictureOfTheDay'
import { todoApi } from '../api/todo'

export const store = configureStore({
  reducer: {
    [pictureOfTheDayApi.reducerPath]: pictureOfTheDayApi.reducer,
    [todoApi.reducerPath]: todoApi.reducer,
  },

  middleware: (getDefaultMiddleware) =>
    getDefaultMiddleware()
      .concat(todoApi.middleware, pictureOfTheDayApi.middleware)
})
