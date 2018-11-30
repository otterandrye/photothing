// @flow

import * as React from "react";
import { type Album } from "../album/AlbumPreview";
import Gallery from "./Gallery";
// eslint-disable-next-line
import css from "../Page.css"; // shared between both apps

type Manifest = {|
  +styles: string[],
  +scripts: string[],
  +hash: string,
  +album: Album,
  +api: string,
|};

export default (props: Manifest) => (
  <React.Fragment>
    <head lang="en">
      <meta charSet="utf-8" />
      <title>Chroma Gallery</title>
      {props.styles.map(style => (
        <link href={style} rel="stylesheet" key={style} />
      ))}
    </head>
    <body lang="en">
      <Gallery hash={props.hash} api={props.api} album={props.album} />
      {/* eslint-disable react/no-danger */}
      <script
        dangerouslySetInnerHTML={{
          __html: `MANIFEST=${JSON.stringify(props)};`,
        }}
      />
      {/* eslint-enable react/no-danger */}
      {props.scripts.map(script => <script src={script} key={script} async />)}
    </body>
  </React.Fragment>
);
