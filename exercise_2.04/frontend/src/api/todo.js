import { createApi, fetchBaseQuery } from '@reduxjs/toolkit/query/react'

export const todoApi = createApi({
  reducerPath: 'todoApi',
  baseQuery: fetchBaseQuery({
    baseUrl: '/api/todo',
  }),
  endpoints: (builder) => ({
    getTodos: builder.query({
      query: () => '',
      providesTags: (response) => response.map(({ id }) => ({ type: 'TODO', id })).concat('TODO'),
    }),
    createTodo: builder.mutation({
      query: ({ text }) => ({
        url: '',
        method: 'POST',
        body: { text },
      }),
      invalidatesTags: ['TODO']
    })
  })
})

export const { useGetTodosQuery, useCreateTodoMutation } = todoApi
