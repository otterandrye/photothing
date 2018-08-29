// @flow

import * as React from "react";
import Api, { type RequestState } from "./Api";

type PhotosApiResponse = {|
  +items: Array<{| +uuid: string, +url: string |}>,
|};

export default () => (
  <Api path="/api/photos">
    {({ data }: RequestState<PhotosApiResponse>) => {
      if (data) {
        return data.items.map(photo => (
          <img src={photo.url} alt={photo.uuid} width="250" key={photo.uuid} />
        ));
      }
      return null;
    }}
  </Api>
);
