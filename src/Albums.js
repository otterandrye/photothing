// @flow

import * as React from "react";
import Api, { type RequestState } from "./Api";
import AlbumPreview, { type Album } from "./album/AlbumPreview";
import AlbumToolbar from "./album/AlbumToolbar";

type AlbumsApiResponse = {|
  +items: Array<Album>,
|};

export default () => (
  <Api path="/api/albums">
    {({ data }: RequestState<AlbumsApiResponse>) => {
      let list = null; // TODO: make a no albums placeholder
      if (data) {
        list = data.items.map(album => (
          <AlbumPreview key={album.id} album={album} />
        ));
      } else {
        list = <span>Loading...</span>;
      }
      return (
        <div>
          <AlbumToolbar />
          <div>{list}</div>
        </div>
      );
    }}
  </Api>
);
