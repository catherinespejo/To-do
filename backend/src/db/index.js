require('dotenv').config();

const { Pool } = require('pg');

// nos conectamos con la variable que agregamos en el docker-compose en la sección de environment
// para el servicio de "base_de_datos" que tiene el formato de:
// - postgres://username:password@hostname:port/database
const urlBaseDeDatos = process.env.DATABASE_URL;

const pool = new Pool({
    connectionString: urlBaseDeDatos,
});

module.exports = {
  query: (text, params) => pool.query(text, params),
};