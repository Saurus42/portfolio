import React from 'react';

interface props {}
interface state {}

class Footer extends React.Component<props, state> {
  constructor( props: props ) {
    super( props );
  }

  render() {
    return (
      <footer className="row">
        <div className="col-sm-12 footer">
          <div className="row">
            <div className="element-widget col-sm">
              <p>Dane kontaktowe:</p>
              <ul>
                <li>tel. kom.: +48 608 673 146</li>
                <li>e-mail: mateusz.krasuski.358@gmail.com</li>
              </ul>
            </div>
            <div className="element-widget col-sm">
              <p>Repozytoriom:</p>
              <p><a href="https://github.com/Saurus42">https://github.com/Saurus42</a></p>
            </div>
          </div>
        </div>
      </footer>
    );
  }
}

export default Footer;