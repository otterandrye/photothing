// @flow

import * as React from "react";
import "./App.css";
import EditorPreview from "./EditorPreview";
import MultiUploader from "./MultiUploader";

type State = {|
  selected: File | null,
|};

export default class App extends React.Component<{||}, State> {
  state: State = { selected: null };
  render() {
    return (
      <React.Fragment>
        {this.state.selected && <EditorPreview input={this.state.selected} />}
        <MultiUploader edit={file => this.setState({ selected: file })} />
      </React.Fragment>
    );
  }
}
