// @flow

import * as React from "react";
import "./App.css";
import EditorPreview from "./EditorPreview";
import Login from "./Login";
import MultiUploader from "./MultiUploader";
import PhotoList from "./PhotoList";
import type { AuthContext } from "./api";

type State = {|
  selected: File | null,
  auth: AuthContext | null,
|};

type Props = {|
  api: string,
|};

export default class App extends React.Component<Props, State> {
  state: State = {
    // The selected file for the editor.
    selected: null,

    // The current auth context
    auth: null,
  };

  componentDidMount() {
    // Check localStorage for a complete auth context...
    const authContext = {
      email: localStorage.getItem("email"),
      header: localStorage.getItem("header"),
      token: localStorage.getItem("token"),
    };

    if (authContext.email && authContext.header && authContext.token) {
      // $FlowFixMe: not getting the refinement right, here.
      this.setState({ auth: authContext });
    }
  }

  authenticate = (authContext: AuthContext) => {
    localStorage.setItem("header", authContext.header);
    localStorage.setItem("token", authContext.token);
    localStorage.setItem("email", authContext.email);
    this.setState({ auth: authContext });
  };

  logout = () => {
    localStorage.clear();
    this.setState({ auth: null });
  };

  render() {
    return (
      <React.Fragment>
        <Login
          api={this.props.api}
          authContext={this.state.auth}
          authenticate={this.authenticate}
          logout={this.logout}
        />
        {this.state.auth && (
          <React.Fragment>
            {this.state.selected && (
              <EditorPreview
                input={this.state.selected}
                height={264}
                width={399}
                scale={1}
              />
            )}
            <MultiUploader
              api={this.props.api}
              authContext={this.state.auth}
              edit={file => this.setState({ selected: file })}
            />
            <PhotoList api={this.props.api} authContext={this.state.auth} />
          </React.Fragment>
        )}
      </React.Fragment>
    );
  }
}
