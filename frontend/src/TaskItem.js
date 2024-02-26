import React from 'react';
import axios from 'axios';

function TaskItem({ task, fetchTasks }) {
  const toggleCompleted = async () => {
    await axios.put(`/api/tasks/${task.id}`, {
      ...task,
      completed: !task.completed,
    });
    fetchTasks();
  };

  const deleteTask = async () => {
    await axios.delete(`/api/tasks/${task.id}`);
    fetchTasks();
  };

  return (
    <li>
      <span style={{ textDecoration: task.completed ? 'line-through' : 'none' }}>
        {task.title}
      </span>
      <button onClick={toggleCompleted}>
        {task.completed ? 'Undo' : 'Complete'}
      </button>
      <button onClick={deleteTask}>Delete</button>
    </li>
  );
}

export default TaskItem;
