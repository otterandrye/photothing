// @flow

import * as React from "react";
import type { Route } from "../routes";

type AuthContext = {|
  email: string,
  header: string,
  token: string,
|};

type Props = {|
  +api: string,
  +authenticate: AuthContext => void,
  +navigate: Route => void,
|};

type State = {|
  email: string,
  password: string,
  message: string,
|};

export default class Login extends React.Component<Props, State> {
  state: State = { email: "", password: "", message: "Please Login" };

  setEmail = (evt: SyntheticInputEvent<*>) =>
    this.setState({ email: evt.target.value });
  setPassword = (evt: SyntheticInputEvent<*>) =>
    this.setState({ password: evt.target.value });

  submit = (evt: SyntheticEvent<*>) => {
    evt.preventDefault();
    fetch(`${this.props.api}/api/login`, {
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
          res.json().then(login => {
            this.setState({ message: "Success!" });
            this.props.authenticate({
              header: login.header,
              token: login.pt_auth,
              email: login.email,
            });
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
            autoComplete="current-password"
            minLength={8}
            required
          />
          <button type="submit">Login</button>
          <button
            type="button"
            onClick={() => this.props.navigate({ page: "FORGOT_PASSWORD" })}
          >
            Forgot Password
          </button>
        </form>
      </React.Fragment>
    );
  }
}
