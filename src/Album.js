// @flow

import * as React from "react";

type Props = {| +id: string, +photoId: string | null |};

export default (props: Props) => {
  const details = props.photoId || "from the top";
  return (
    <div>
      Album: {props.id}, {details}
    </div>
  );
};
