import { useState } from 'react'
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
        <li>{item}</li>
      ))}
    </ul>
  )
}

export const TodoWidget = () => {
  const [items, setItems] = useState([
    'Implement rest of the TODO functionality',
    'Pet dogs',
    'Enjoy life'
  ])

  const handleNewItem = (item) => {
    setItems(items => [...items, item])
  }

  return (
    <div className="todo-container">
      <h3>Todo List</h3>
      <TodoList items={items} />
      <TodoInput onNewItem={handleNewItem} />
    </div>
  )
};
