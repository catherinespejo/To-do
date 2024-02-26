const express = require('express');
const router = express.Router();
const pool = require('../db/index');

// Obtener todas las tareas
router.get('/', async (req, res) => {
  const { rows } = await pool.query('SELECT * FROM tasks ORDER BY id ASC');
  res.status(200).json(rows);
});

// Añadir una nueva tarea
router.post('/', async (req, res) => {
  const { title, completed } = req.body;
  const { rows } = await pool.query('INSERT INTO tasks (title, completed) VALUES ($1, $2) RETURNING *', [title, completed]);
  res.status(201).json(rows[0]);
});

// Actualizar una tarea existente
router.put('/:id', async (req, res) => {
  const { id } = req.params;
  const { title, completed } = req.body;
  try {
    const { rows } = await pool.query(
      'UPDATE tasks SET title = $1, completed = $2 WHERE id = $3 RETURNING *',
      [title, completed, id]
    );
    if (rows.length > 0) {
      res.json(rows[0]);
    } else {
      res.status(404).json({ message: "La tarea no existe." });
    }
  } catch (err) {
    res.status(500).json({ message: err.message });
  }
});

// Eliminar una tarea
router.delete('/:id', async (req, res) => {
  const { id } = req.params;
  try {
    const { rows } = await pool.query(
      'DELETE FROM tasks WHERE id = $1 RETURNING *',
      [id]
    );
    if (rows.length > 0) {
      res.json({ message: "Tarea eliminada.", task: rows[0] });
    } else {
      res.status(404).json({ message: "La tarea no existe." });
    }
  } catch (err) {
    res.status(500).json({ message: err.message });
  }
});

module.exports = router;
