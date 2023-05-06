import React from 'react';
// @ts-ignore
import { Link } from 'react-router-dom';

interface props {}
interface state {}

class Home extends React.Component<props, state> {
  constructor( props: props ) {
    super( props );
  }

  render() {
    return (
      <>
        <div className="row">
          <div className="col-sm-12 content">
            <p>Nazywam się Mateusz Krasuski</p>
            <p>Specjalizuję się w językach takich jak JavaScript/TypeScript do tego uczę się języka Rust.</p>
            <p>Stosowane technologie dla JavaScript/TypeScript to:</p>
            <ul>
              <li>React.js</li>
              <li>Node.js</li>
              <li>Koa.js</li>
              <li>Express.js</li>
              <li>Vue.js</li>
            </ul>
            <p>Znam również języki takie jak:</p>
            <ul>
              <li>PHP</li>
              <li>C</li>
              <li>C++</li>
              <li>C#</li>
              <li>Podstawy Java</li>
            </ul>
            <p>Zajmuję się frontend-em jak i backend-em.</p>
            <p>Wszystkie moje projekty są do wglądu w <a href="https://github.com/Saurus42">moim repozytorium</a> oraz w zakładce <Link to="/projects">Moje projekty</Link>.
            </p>
          </div>
        </div>
      </>
    );
  }
}

export default Home;