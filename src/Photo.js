// @flow

import * as React from "react";

type Props = {|
  +alt: string,
  +src: string,
  +className: string,
  +error: null | (() => React.Node),
  +onClick?: () => void,
|};

type State = {| status: "LOADING" | "ERROR" | "LOADED" |};

export default class Photo extends React.Component<Props, State> {
  static defaultProps = {
    error: null,
    onClick: undefined,
  };

  state: State = { status: "LOADING" };

  onError = () => {
    this.setState({ status: "ERROR" });
  };

  onLoad = () => {
    this.setState({ status: "LOADED" });
  };

  render() {
    if (this.state.status === "ERROR") {
      return this.props.error;
    }

    return (
      <img
        src={this.props.src}
        alt={this.props.alt}
        className={this.props.className}
        onError={this.onError}
        onLoad={this.onLoad}
        onClick={this.props.onClick}
        style={{ display: this.state.status === "LOADED" ? "flex" : "none" }}
      />
    );
  }
}
