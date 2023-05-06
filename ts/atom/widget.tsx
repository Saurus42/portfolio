import React from 'react';

interface props {
  children: any
}
interface state {}

class Widget extends React.Component<props, state> {
  constructor( props: props ) {
    super( props );
  }

  render() {
    return (
      <div className="row">
        <div className="col-sm-12 widget">
          <div className="row">
            { this.props.children }
          </div>
        </div>
      </div>
    );
  }
}

export default Widget;
