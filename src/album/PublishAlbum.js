// @flow

import * as React from "react";
import { connect } from "react-redux";

type Props = {|
  albumId: number,
  api: string,
  headers: { [string]: string },
|};

type State = {|
  +pending: boolean,
|};

class PublishAlbum extends React.Component<Props, State> {
  state: State = { pending: false };

  submit = () => {
    this.setState({ pending: true });
    fetch(`${this.props.api}/api/albums/${this.props.albumId}/publish`, {
      headers: this.props.headers,
      method: "POST",
    }).then(() => this.setState({ pending: false }));
  };

  render() {
    const text = this.state.pending ? "..." : "Publish album";
    return (
      <button type="button" onClick={this.submit}>
        {text}
      </button>
    );
  }
}

export default connect(({ api }) => ({ api: api.host, headers: api.headers }))(
  PublishAlbum,
);
