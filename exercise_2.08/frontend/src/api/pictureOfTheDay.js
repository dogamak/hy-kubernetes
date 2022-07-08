import { createApi, fetchBaseQuery } from '@reduxjs/toolkit/query/react'

export const pictureOfTheDayApi = createApi({
  reducerPath: 'pictureOfTheDayApi',
  baseQuery: fetchBaseQuery({
    baseUrl: '/api',
  }),
  endpoints: (builder) => ({
    getPictureOfTheDay: builder.query({
      query: () => 'picture-of-the-day',
    }),
  }),
})

export const {
  useGetPictureOfTheDayQuery,
} = pictureOfTheDayApi
