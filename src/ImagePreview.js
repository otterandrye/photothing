// @flow

import * as React from "react";
import Loader from "./Loader";
import css from "./ImagePreview.css";

type Props = {
  file: File,
};

type State = {
  dataUrl: null | string,
};

const isWebSafeImage = (type: string) =>
  type === "image/jpeg" ||
  type === "image/png" ||
  type === "image/gif" ||
  type === "image/svg+xml";

export default class ImagePreview extends React.Component<Props, State> {
  state: State = { dataUrl: null };

  componentDidMount() {
    if (isWebSafeImage(this.props.file.type)) {
      const reader = new FileReader();
      reader.onload = evt => this.setState({ dataUrl: evt.target.result });
      reader.readAsDataURL(this.props.file);
    }
  }

  render() {
    const { dataUrl } = this.state;
    if (dataUrl !== null) {
      return (
        <img src={dataUrl} alt={this.props.file.name} className={css.preview} />
      );
    }
    if (isWebSafeImage(this.props.file.type)) {
      return <Loader />;
    }
    return <span className={css.cannotDisplay}>?</span>;
  }
}
