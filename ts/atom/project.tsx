import React from 'react';

interface props {
  name: string,
  imageUrl: string,
  description: string,
  link: string
}
interface state {}

class Project extends React.Component<props, state> {
  constructor( props: props ) {
    super( props );
  }

  render() {
    return (
      <a href={this.props.link} className="project col-sm-3">
        <article className="row">
          <header>
            <img src={this.props.imageUrl} className="col-sm-12" alt={this.props.name} />
          </header>
          <section>
            <p>Projekt: {this.props.name}</p>
            <p>{this.props.description}</p>
          </section>
        </article>
      </a>
    );
  }
}

export default Project;