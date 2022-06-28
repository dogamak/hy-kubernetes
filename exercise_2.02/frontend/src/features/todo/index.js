import { useState } from 'react'
import { useCreateTodoMutation, useGetTodosQuery } from '../../api/todo'
import './style.css'

const TodoInput = ({ onNewItem }) => {
  const [value, setValue] = useState('')

  const handleSubmit = () => {
    onNewItem(value);
    setValue('');
  }

  return (
    <div className="todo-input">
      <input type="text" value={value} onChange={(evt) => setValue(evt.target.value.substring(0, 140))} />
      <button onClick={handleSubmit}>Add</button>
    </div>
  )
}

const TodoList = ({ items }) => {
  return (
    <ul className="todo-list">
      {items.map((item) => (
        <li key={item.id}>{item.text}</li>
      ))}
    </ul>
  )
}

export const TodoWidget = () => {
  const { data: items, isLoading } = useGetTodosQuery()
  const [createTodo] = useCreateTodoMutation()

  const handleNewItem = (item) => {
    createTodo({
      text: item,
    })
  }

  return (
    <div className="todo-container">
      <h3>Todo List</h3>
      {isLoading && <p>Loading...</p>}
      <TodoList items={items ?? []} />
      <TodoInput onNewItem={handleNewItem} />
    </div>
  )
};
