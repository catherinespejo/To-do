import React, { useEffect, useState } from 'react';
import { apiClient } from './axios';
import TaskList from './TaskList';
import AddTaskForm from './AddTaskForm';

function App() {
  const [tasks, setTasks] = useState([]);

  useEffect(() => {
    fetchTasks();
  }, []);

  const fetchTasks = async () => {
    try {
      const response = await apiClient.get('/api/tasks/');
      console.log('test3')
      console.log({ test: response.data })
      setTasks(response.data);
    } catch (error) {
      console.log('test5545454')
      console.log({ error: error.response });
    }
  };

  const addTask = async (title) => {
    await apiClient.post('/api/tasks/', { title, completed: false });
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
