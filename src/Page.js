// @flow

import * as React from "react";
import { Provider } from "react-redux";
import { createStore, type Store } from "redux";
import App from "./App";
import History from "./History";
import Authenticator from "./Authenticator";
import { reducer, type State, type Action } from "./State";
// eslint-disable-next-line
import css from "./Page.css";

type Manifest = {|
  +styles: string[],
  +scripts: string[],
  +state: State,
|};

export default class Page extends React.Component<Manifest> {
  constructor(props: Manifest) {
    super(props);
    this.store = createStore(reducer, props.state);
  }

  store: Store<State, Action>;

  render() {
    return (
      <Provider store={this.store}>
        <React.Fragment>
          <head lang="en">
            <meta charSet="utf-8" />
            <title>Chroma</title>
            {this.props.styles.map(style => (
              <link href={style} rel="stylesheet" key={style} />
            ))}
          </head>
          <body lang="en">
            <History />
            <Authenticator />
            <App />
            {/* eslint-disable react/no-danger */}
            <script
              dangerouslySetInnerHTML={{
                __html: `MANIFEST=${JSON.stringify(this.props)};`,
              }}
            />
            {/* eslint-enable react/no-danger */}
            {this.props.scripts.map(script => (
              <script src={script} key={script} async />
            ))}
          </body>
        </React.Fragment>
      </Provider>
    );
  }
}
