import React from 'react';
import { useOutlet } from '../../node_modules/react-router-dom/dist/index';
import Menu from '../molecule/menu';
import Footer from '../atom/footer';

interface props {}

function Layout( props: props ) {
  const outlet = useOutlet()
  return (<div className="container-sm">
    <Menu names={ [ "O mnie", "Moje projekty" ] } urls={ [ '/', '/projects' ] } />
    { outlet }
    <Footer />
  </div>);
}

export default Layout;
