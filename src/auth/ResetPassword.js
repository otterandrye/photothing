// @flow

import * as React from "react";
import { connect } from "react-redux";

import { navigate } from "../State";
import type { Route } from "../routes";

type Props = {|
  +api: string,
  +email: string,
  +id: string,
  +navigate: Route => void,
|};

type State = {|
  password: string,
  success: boolean,
  message: string,
|};

class ResetPassword extends React.Component<Props, State> {
  state: State = {
    password: "",
    success: false,
    message: "Enter a new password",
  };

  setPassword = (evt: SyntheticInputEvent<*>) =>
    this.setState({ password: evt.target.value });

  submit = (evt: SyntheticEvent<*>) => {
    evt.preventDefault();
    fetch(`${this.props.api}/api/reset_password/${this.props.id}`, {
      method: "PUT",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        email: this.props.email,
        password: this.state.password,
      }),
    })
      .then(res => {
        if (res.ok) {
          this.setState({
            success: true,
            message: "Success! Please log in.",
          });
        } else {
          this.setState({ message: `Error code ${res.status}` });
        }
      })
      .catch(err => {
        this.setState({ message: `Error: ${err}` });
      });
  };
  render() {
    return (
      <React.Fragment>
        <div>{this.state.message}</div>
        {!this.state.success && (
          <form onSubmit={this.submit}>
            <input
              type="password"
              value={this.state.password}
              onChange={this.setPassword}
              autoComplete="current-password"
              minLength={8}
              required
            />
            <button type="submit">Finish Password Reset</button>
          </form>
        )}
        {this.state.success && (
          <button
            type="button"
            onClick={() => this.props.navigate({ page: "LOGIN" })}
          >
            Log In
          </button>
        )}
      </React.Fragment>
    );
  }
}

export default connect(
  state => ({ api: state.api.host }),
  { navigate },
)(ResetPassword);
