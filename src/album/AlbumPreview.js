// @flow

import * as React from "react";
import { connect } from "react-redux";

import { navigate } from "../State";
import type { Route } from "../routes";
import type { Page } from "../Pagination";
import type { PhotoInfo } from "../PhotoList";

import PublishAlbum from "./PublishAlbum";

type AlbumEntry = {|
  caption: ?string,
  ordering: ?number,
  photo: PhotoInfo,
|};
type Album = {|
  +id: number,
  +name: string,
  +photos: Page<AlbumEntry>,
|};
type Props = {| +album: Album, +navigate: Route => void |};

export type { Album, AlbumEntry };

const AlbumPreview = (props: Props) => (
  <div>
    <span>
      {props.album.name} w/ {props.album.photos.items.length} photos
    </span>
    <button
      type="button"
      onClick={() =>
        props.navigate({
          page: "ALBUM",
          albumId: props.album.id,
          photoId: null,
        })
      }
    >
      View album
    </button>
    <PublishAlbum albumId={props.album.id} />
  </div>
);

export default connect(
  () => ({}),
  { navigate },
)(AlbumPreview);
