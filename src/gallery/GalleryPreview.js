// @flow

import * as React from "react";
import Api, { type RequestState } from "../Api";
import { type Album } from "../album/AlbumPreview";

type UrlFriendlyAlbum = {|
  album_id: number,
  hash: string,
  active: boolean,
|};

type Props = {| +gallery: UrlFriendlyAlbum |};

export type { UrlFriendlyAlbum };

export default (props: Props) => (
  <Api path={`/api/published/${props.gallery.hash}`}>
    {({ data }: RequestState<Album>) => {
      if (data) {
        const link = `/view/${props.gallery.hash}`;
        return (
          <div>
            <div>
              Album name: {data.name}, album id: {props.gallery.album_id},
              contains {data.photos.items.length} photos
            </div>
            <div>
              <span>Gallery link: </span>
              <a href={link} target="_blank" rel="noreferrer noopener">
                html link (TODO)
              </a>
            </div>
          </div>
        );
      }
      return null;
    }}
  </Api>
);
