/*
 * @id     cog
 * @do     demarrer_site_cog
 * @role   orchestration
 * @layer  ui
 * @human  Racine de miyukini-cog : monte le site officiel qui présente le projet Cog (le cœur cognitif de l'écosystème Miyukini).
 */
import React from 'react';
import ReactDOM from 'react-dom/client';
import { BrowserRouter } from 'react-router-dom';
import App from './App';
import './index.css';

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <BrowserRouter>
      <App />
    </BrowserRouter>
  </React.StrictMode>
);
