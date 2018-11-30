// @flow

import * as React from "react";
import { type Album } from "../album/AlbumPreview";
import Header from "./Header";
import css from "./Gallery.css";

type Props = {|
  hash: string,
  api: string,
  album: Album,
|};

export default (props: Props) => (
  <div className={css.galleryPage}>
    <Header text={props.album.name} />
    <div className={css.container}>
      {props.album.photos.items.length === 0 ? (
        <h2>No photos here yet</h2>
      ) : null}
      {props.album.photos.items.map(albumEntry => {
        const { photo } = albumEntry;
        return (
          <div key={photo.id} className={css.item}>
            <img className={css.pic} src={photo.url} alt={albumEntry.caption} />
          </div>
        );
      })}
    </div>
  </div>
);
