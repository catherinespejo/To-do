require('dotenv').config();
const express = require('express');
const taskRoutes = require('./routes/tasks');
const cors = require('cors');

const app = express();
const PORT = process.env.PORT || 6001;

app.use(express.json());

// Se utiliza libreria cors para permitir que el frontend se conecte al backend de manera local
app.use(
  cors({ origin: process.env.FRONTEND_URL || 'http://localhost:3000' }),
);

app.use('/api/tasks', taskRoutes);

app.listen(PORT, () => {
  console.log(`Servidor esta corriendo en http://0.0.0.0:${PORT}`);
});
