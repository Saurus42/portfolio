import React from 'react';
import MenuItem from '../atom/menuItem';

interface props {
  names: string[]
  urls: string[]
}
interface state {}

class Menu extends React.Component<props, state> {
  constructor( props: props ) {
    super( props );
  }

  static index = 0;

  render() {
    let names = [];
    let urls = [];
    for( const name of this.props.names ) {
      names.push( name );
    }
    for( const url of this.props.urls ) {
      urls.push( url );
    }
    let items = [];
    for( let i = 0; i < names.length; i++ ) {
      items.push( <MenuItem key={`menu-${Menu.index++}`} urlLink={ urls[i] } nameLink={ names[i] } /> );
    }
    return (
      <nav className="row">
        <div className="col-sm-12 navigation">
          <div className="row">
            { items }
          </div>
        </div>
      </nav>
    );
  }
}

export default Menu;