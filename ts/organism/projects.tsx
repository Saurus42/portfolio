import Project from '../atom/project';
import Widget from '../atom/widget';
import React from 'react';

interface product {
  name: string,
  imageUrl: string,
  description: string,
  link: string
}

interface props {}
interface state {
  products: product[]
}

class Projects extends React.Component<props, state> {
  constructor( props: props ) {
    super( props );
    this.state = {
      products: []
    }
  }

  localData(): product[] {
    let products = sessionStorage.getItem( 'products' );
    if( products === null )
      products = '[]';
    return JSON.parse( products );
  }

  render() {
    const JSONProducts = this.localData();
    const widgets = [];
    for( const product of JSONProducts )
      widgets.push( <Project name={ product.name } link={ product.link } imageUrl={ product.imageUrl } description={ product.description } /> )
    return (
      <>
        <Widget>
          {widgets}
        </Widget>
      </>
    );
  }

  async componentDidMount() {
    if( this.state.products.length === 0 || sessionStorage.getItem( 'products' ) === null ) {
      const products = await fetch( '/portfolio/assets/products.json' ).then<product[]>( res => res.json() );
      this.setState( { products } );
    }
  }

  componentDidUpdate(prevProps: Readonly<props>, prevState: Readonly<state>, snapshot?: any): void {
    sessionStorage.setItem( 'products', JSON.stringify( this.state.products ) );
  }
}

export default Projects;
