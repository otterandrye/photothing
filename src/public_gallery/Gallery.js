// @flow

import * as React from "react";
import { type Album } from "../album/AlbumPreview";
import css from "./Gallery.css";

type Props = {|
  hash: string,
  api: string,
  album: Album,
|};

export default (props: Props) => (
  <div className={css.container}>
    <h1>public album, hash={props.hash}</h1>
    {props.album.photos.items.length === 0 ? <h2>No photos here yet</h2> : null}
    {props.album.photos.items.map(albumEntry => {
      const { photo } = albumEntry;
      return (
        <div>
          <img
            key={photo.id}
            width="500px"
            src={photo.url}
            alt={albumEntry.caption}
          />
        </div>
      );
    })}
  </div>
);
