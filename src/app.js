// @flow

import * as React from "react";
import classes from "./app.css";

type Props = {|
  +styles: string[],
  +scripts: string[],
|};

export default ({ styles, scripts }: Props) => (
  <React.Fragment>
    <head lang="en">
      <meta charSet="utf-8" />
      <title>title</title>
      {styles.map(style => <link href={style} rel="stylesheet" key={style} />)}
    </head>
    <body lang="en">
      <div className={classes.pretty}>body text</div>
      {/* eslint-disable react/no-danger */}
      <script
        dangerouslySetInnerHTML={{
          __html: `SCRIPTS = ${JSON.stringify(
            scripts,
          )}; STYLES = ${JSON.stringify(styles)};`,
        }}
      />
      {/* eslint-enable react/no-danger */}
      {scripts.map(script => <script src={script} key={script} async />)}
    </body>
  </React.Fragment>
);
