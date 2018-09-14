// @flow

import * as React from "react";

import EditorPreview from "./EditorPreview";
import MultiUploader from "./MultiUploader";
import PhotoList from "./PhotoList";

type State = {|
  selected: File | null,
|};

export default class Library extends React.Component<{||}, State> {
  state: State = { selected: null };
  render() {
    return (
      <React.Fragment>
        {this.state.selected && (
          <EditorPreview
            input={this.state.selected}
            height={264}
            width={399}
            scale={1}
          />
        )}
        <MultiUploader edit={file => this.setState({ selected: file })} />
        <PhotoList />
      </React.Fragment>
    );
  }
}
