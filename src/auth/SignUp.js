// @flow

import * as React from "react";
import type { Route } from "../routes";

type Props = {|
  +api: string,
  +navigate: Route => void,
|};

type State = {|
  email: string,
  password: string,
  message: string,
  success: boolean,
|};

export default class SignUp extends React.Component<Props, State> {
  state: State = {
    email: "",
    password: "",
    message: "Please Sign Up",
    success: false,
  };

  setEmail = (evt: SyntheticInputEvent<*>) =>
    this.setState({ email: evt.target.value });
  setPassword = (evt: SyntheticInputEvent<*>) =>
    this.setState({ password: evt.target.value });

  submit = (evt: SyntheticEvent<*>) => {
    evt.preventDefault();
    fetch(`${this.props.api}/api/register`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        email: this.state.email,
        password: this.state.password,
      }),
    })
      .then(res => {
        if (res.ok) {
          res.json().then(() => {
            this.setState({
              message: "Success! Please log in.",
              success: true,
            });
          });
        } else {
          res.json().then(error => {
            this.setState({ message: `Registration failed: ${error.message}` });
          });
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
              type="email"
              value={this.state.email}
              onChange={this.setEmail}
              autoComplete="email"
              required
            />
            <input
              type="password"
              value={this.state.password}
              onChange={this.setPassword}
              autoComplete="new-password"
              minLength={8}
              required
            />
            <button type="submit">Sign Up</button>
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
