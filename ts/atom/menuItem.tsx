import React from 'react';
// @ts-ignore
import { NavLink } from 'react-router-dom';

interface props {
  nameLink: string
  urlLink: string
}
interface state {}

class MenuItem extends React.Component<props, state> {
  constructor( props: props ) {
    super( props );
  }

  render() {
    return (
      <NavLink className="item-navigator col-sm text-center" to={this.props.urlLink}>{this.props.nameLink}</NavLink>
    );
  }
}

export default MenuItem;