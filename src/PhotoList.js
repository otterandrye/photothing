// @flow

import * as React from "react";
import type { AuthContext } from "./api";

type Props = {|
  +api: string,
  +authContext: AuthContext,
|};

type State = {|
  photos: Array<{ uuid: string, cdn_url: string }>,
|};

export default class PhotoList extends React.Component<Props, State> {
  state = {
    photos: [],
  };

  componentDidMount() {
    fetch(`${this.props.api}/api/photos`, {
      headers: {
        [this.props.authContext.header]: this.props.authContext.token,
      },
    })
      .then(res => res.json())
      .then(res => {
        this.setState({ photos: res.items });
      });
  }
  render() {
    return this.state.photos.map(photo => (
      <img src={photo.cdn_url} alt={photo.uuid} />
    ));
  }
}
