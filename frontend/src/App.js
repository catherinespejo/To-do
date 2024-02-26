import React, { useEffect, useState } from 'react';
import axios from 'axios';
import TaskList from './TaskList';
import AddTaskForm from './AddTaskForm';

function App() {
  const [tasks, setTasks] = useState([]);

  useEffect(() => {
    fetchTasks();
  }, []);

  const fetchTasks = async () => {
    const response = await axios.get('/api/tasks');
    setTasks(response.data);
  };

  const addTask = async (title) => {
    await axios.post('/api/tasks', { title, completed: false });
    fetchTasks();
  };

  return (
    <div className="App">
      <h1>Todo List</h1>
      <AddTaskForm onAddTask={addTask} />
      <TaskList tasks={tasks} fetchTasks={fetchTasks} />
    </div>
  );
}

export default App;
