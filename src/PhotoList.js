// @flow

import * as React from "react";
import Api, { type RequestState } from "./Api";

type Photo = {|
  +id: string,
  +uuid: string,
  +url: string,
  +attributes: { filename: string },
|};
type PhotosApiResponse = {|
  +items: Array<Photo>,
|};

type Props = {|
  onSelect: string => void,
|};

const PhotoList = (props: Props) => (
  <Api path="/api/photos">
    {({ data }: RequestState<PhotosApiResponse>) => {
      if (data) {
        return data.items.map(photo => (
          <img
            src={photo.url}
            alt={photo.uuid}
            width="250"
            key={photo.uuid}
            onClick={() => props.onSelect(photo.id)}
          />
        ));
      }
      return null;
    }}
  </Api>
);

PhotoList.defaultProps = {
  onSelect: () => {},
};

export default PhotoList;
