import { configureStore } from '@reduxjs/toolkit'
import { pictureOfTheDayApi } from '../api/pictureOfTheDay'

export const store = configureStore({
  reducer: {
    [pictureOfTheDayApi.reducerPath]: pictureOfTheDayApi.reducer,
  },
})
