// @flow

import * as React from "react";
import Loader from "./Loader";
import css from "./ImagePreview.css";

type Props = {
  file: File,
  selected: boolean,
  onClick?: () => void,
};

const isWebSafeImage = (type: string) =>
  type === "image/jpeg" ||
  type === "image/png" ||
  type === "image/gif" ||
  type === "image/svg+xml";

const ImagePreview = ({ file, selected, onClick }: Props) => {
  // $FlowFixMe: This is an experimental API.
  const [dataUrl, setDataUrl] = React.useState(null);

  // $FlowFixMe: This is an experimental API.
  const ref = React.useRef();

  // $FlowFixMe: This is an experimental API.
  React.useMutationEffect(() => {
    if (selected && ref.current) {
      ref.current.scrollIntoView();
    }
  });

  // $FlowFixMe: This is an experimental API.
  React.useEffect(
    () => {
      const reader = new FileReader();
      reader.onload = evt => setDataUrl(evt.target.result);
      reader.readAsDataURL(file);
    },
    [file],
  );

  if (dataUrl !== null) {
    return (
      <img
        src={dataUrl}
        alt={file.name}
        className={`${css.preview}${selected ? ` ${css.selected}` : ""}`}
        onClick={onClick}
        ref={ref}
      />
    );
  }

  if (isWebSafeImage(file.type)) {
    return <Loader ref={ref} />;
  }

  return (
    <span ref={ref} className={css.cannotDisplay}>
      ?
    </span>
  );
};

ImagePreview.defaultProps = {
  selected: false,
  onClick: undefined,
};

export default ImagePreview;
