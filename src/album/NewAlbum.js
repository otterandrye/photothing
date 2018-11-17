// @flow

import * as React from "react";
import { connect } from "react-redux";

type Props = {|
  api: string,
  headers: { [string]: string },
|};

type State = {|
  +name: ?string,
  +pending: boolean,
|};

class NewAlbum extends React.Component<Props, State> {
  state: State = { name: null, pending: false };

  submit = () => {
    const name = this.state.name || "New Album";
    this.setState({ pending: true });
    fetch(`${this.props.api}/api/albums?name=${name}`, {
      method: "POST",
      headers: this.props.headers,
    }).then(() => this.setState({ pending: false }));
    // TODO: how to trigger a re-fetch of albums?
  };

  render() {
    const text = this.state.pending ? "Creating" : "New album";
    return (
      <React.Fragment>
        <button onClick={this.submit} type="button">
          {text}
        </button>
      </React.Fragment>
    );
  }
}

export default connect(({ api }) => ({ api: api.host, headers: api.headers }))(
  NewAlbum,
);
