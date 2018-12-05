// @flow

import * as React from "react";
import Api, { type RequestState } from "./Api";
import Photo from "./Photo";
import styles from "./PhotoList.css";

export type PhotoInfo = {|
  +id: string,
  +uuid: string,
  +url: string,
  +attributes: { filename: string },
|};
type PhotosApiResponse = {|
  +items: Array<PhotoInfo>,
|};

type Props = {|
  onSelect: string => void,
|};

const PhotoList = (props: Props) => (
  <Api path="/api/photos">
    {({ data }: RequestState<PhotosApiResponse>) => {
      if (data) {
        return data.items.map(photo => (
          <Photo
            src={photo.url}
            alt={photo.uuid}
            key={photo.uuid}
            className={styles.photo}
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
