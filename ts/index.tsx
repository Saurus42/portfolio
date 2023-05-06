import React from 'react';
import { createRoot } from 'react-dom/client';
import Projects from './organism/projects';
import Layout from './organism/layout';
import Home from './organism/home';
import { BrowserRouter, Route, Routes } from '../node_modules/react-router-dom/dist/index';

window.addEventListener( 'load', function( event ) {
  const container = this.document.createElement( 'div' )
  const root = createRoot( container );
  root.render(
    <BrowserRouter>
      <Routes>
        <Route path="/portfolio/" element={ <Layout /> }>
          <Route index element={ <Home /> } />
          <Route path="projects" element={ <Projects /> } />
        </Route>
      </Routes>
    </BrowserRouter>
  );
  this.document.body.appendChild( container );
} );