import React from 'react';
import TaskItem from './TaskItem';

function TaskList({ tasks, fetchTasks }) {
  return (
    <ul>
      {tasks.map((task) => (
        <TaskItem key={task.id} task={task} fetchTasks={fetchTasks} />
      ))}
    </ul>
  );
}

export default TaskList;
