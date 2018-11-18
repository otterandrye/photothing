// @flow

import * as React from "react";
import Api, { type RequestState } from "./Api";
import { type Album } from "./album/AlbumPreview";

import PhotoPicker from "./PhotoPicker";

type Props = {| +id: number, +photoId: string | null |};

export default (props: Props) => {
  const details = props.photoId || "from the top";
  return (
    <Api path={`/api/albums/${props.id}`}>
      {({ data }: RequestState<Album>) => {
        if (!data) {
          return <div>Loading...</div>;
        }
        return (
          <div>
            <div>
              Album name={data.name} id={props.id}, {details} ({
                data.photos.items.length
              }{" "}
              photos)
            </div>
            <PhotoPicker albumId={props.id} />
          </div>
        );
      }}
    </Api>
  );
};
