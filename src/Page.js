// @flow

import * as React from "react";
import App from "./App";

type Manifest = {|
  +styles: string[],
  +scripts: string[],
|};

export default (manifest: Manifest) => (
  <React.Fragment>
    <head lang="en">
      <meta charSet="utf-8" />
      <title>title</title>
      {manifest.styles.map(style => (
        <link href={style} rel="stylesheet" key={style} />
      ))}
    </head>
    <body lang="en">
      <App />
      {/* eslint-disable react/no-danger */}
      <script
        dangerouslySetInnerHTML={{
          __html: `MANIFEST=${JSON.stringify(manifest)};`,
        }}
      />
      {/* eslint-enable react/no-danger */}
      {manifest.scripts.map(script => (
        <script src={script} key={script} async />
      ))}
    </body>
  </React.Fragment>
);
