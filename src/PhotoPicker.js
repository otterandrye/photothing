// @flow

import * as React from "react";
import { connect } from "react-redux";

import PhotoList from "./PhotoList";

type Props = {|
  +albumId: number,
  +headers: { [string]: string },
  +api: string,
|};
type State = {|
  +selected: Set<string>,
|};

class PhotoPicker extends React.Component<Props, State> {
  state: State = { selected: new Set([]) };

  onSelect = (id: string) => {
    // eslint-disable-next-line react/no-access-state-in-setstate
    const current = new Set(this.state.selected);
    if (current.has(id)) {
      current.delete(id);
    } else {
      current.add(id);
    }
    this.setState({ selected: current });
  };

  onAdd = () => {
    const selectedIds = [];
    this.state.selected.forEach(i => selectedIds.push(parseInt(i, 10)));
    fetch(`${this.props.api}/api/albums/${this.props.albumId}/photos`, {
      headers: {
        "Content-Type": "application/json; charset=utf-8",
        ...this.props.headers,
      },
      method: "PUT",
      body: JSON.stringify(selectedIds),
    }).then(() => this.setState({ selected: new Set([]) }));
  };

  render() {
    const addButton =
      this.state.selected.size > 0 ? (
        <button type="button" onClick={this.onAdd}>
          Add selected
        </button>
      ) : null;
    return (
      <div>
        <div>{addButton}</div>
        <PhotoList onSelect={this.onSelect} />
      </div>
    );
  }
}

export default connect(({ api }) => ({ api: api.host, headers: api.headers }))(
  PhotoPicker,
);
