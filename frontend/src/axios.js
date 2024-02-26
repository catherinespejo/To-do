import axios from 'axios';

export const apiClient = axios.create({
  // Todas las variables de entorno se agregan debajo del nombre "REACT_APP_*"
  baseURL: process.env.REACT_APP_API_URL || 'http://localhost:6001',
  headers: {
    "Content-type": "application/json"
  }
});
