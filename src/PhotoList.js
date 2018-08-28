// @flow

import * as React from "react";
import Api from "./Api";

// photos: Array<{ uuid: string, url: string }>,

export default () => (
  <Api path="/api/photos">
    {({ data }) => {
      if (data && data.items && Array.isArray(data.items)) {
        return data.items.map(photo => (
          <img src={photo.url} alt={photo.uuid} width="250" key={photo.uuid} />
        ));
      }
      return null;
    }}
  </Api>
);
