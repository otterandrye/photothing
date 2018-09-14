// @flow

import * as React from "react";
import App from "./App";
import type { Route } from "./routes";
import { RouteProvider } from "./RouteContext";

type Manifest = {|
  +styles: string[],
  +scripts: string[],
  +api: string,
  +route: Route,
|};

type State = {|
  route: Route,
|};

export default class Page extends React.Component<Manifest, State> {
  constructor(props: Manifest) {
    super(props);
    this.state = { route: props.route };
  }

  navigate = (route: Route) => {
    this.setState({ route });
  };

  render() {
    return (
      <React.Fragment>
        <head lang="en">
          <meta charSet="utf-8" />
          <title>Photothing</title>
          {this.props.styles.map(style => (
            <link href={style} rel="stylesheet" key={style} />
          ))}
        </head>
        <body lang="en">
          <RouteProvider
            value={{
              route: this.state.route,
              navigate: this.navigate,
            }}
          >
            <App
              api={this.props.api}
              navigate={this.navigate}
              route={this.state.route}
            />
          </RouteProvider>
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
    );
  }
}
