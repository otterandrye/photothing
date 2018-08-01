// @flow

import * as React from "react";
import "./App.css";
import EditorPreview from "./EditorPreview";
import Login from "./Login";
import MultiUploader from "./MultiUploader";

type State = {|
  selected: File | null,
|};

type Props = {|
  api: string,
|};

export default class App extends React.Component<Props, State> {
  state: State = { selected: null };
  render() {
    return (
      <React.Fragment>
        <Login api={this.props.api} />
        {this.state.selected && (
          <EditorPreview
            input={this.state.selected}
            height={256}
            width={256}
            scale={1}
          />
        )}
        <MultiUploader
          api={this.props.api}
          edit={file => this.setState({ selected: file })}
        />
      </React.Fragment>
    );
  }
}
