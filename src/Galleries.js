// @flow

import * as React from "react";
import Api, { type RequestState } from "./Api";

import GalleryPreview, {
  type UrlFriendlyAlbum,
} from "./gallery/GalleryPreview";

export default () => (
  <Api path="/api/albums/published">
    {({ data }: RequestState<Array<UrlFriendlyAlbum>>) => {
      if (data) {
        const galleries = data.map(g => (
          <GalleryPreview key={g.hash} gallery={g} />
        ));
        return <div>{galleries}</div>;
      }
      return <div>No Galleries published yet</div>;
    }}
  </Api>
);
