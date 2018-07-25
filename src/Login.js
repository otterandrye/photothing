// @flow

import * as React from "react";

type Props = {|
  api: string,
|};

type State = {|
  email: string,
  password: string,
  message: string | null,
  loggedIn: boolean,
|};

type LoginRegister = {|
  email: string,
  password: string,
|};

export default class Login extends React.Component<Props, State> {
  state: State = { email: "", password: "", message: null, loggedIn: false };

  doLogin = (e: SyntheticEvent<*>) => {
    e.preventDefault();
    const data = this.loginData();
    const xhr = new XMLHttpRequest();
    xhr.open("POST", `${this.props.api}/api/login`);
    xhr.withCredentials = true;
    xhr.setRequestHeader("Content-type", "application/json");
    xhr.onreadystatechange = () => {
      if (xhr.readyState === 4) {
        if (xhr.status === 200) {
          this.setState({
            message: `Welcome, ${data.email}`,
            loggedIn: true,
          });
        } else {
          this.setState({
            message: `Login failed with status=${xhr.status}`,
            loggedIn: false,
          });
        }
      }
    };
    xhr.send(JSON.stringify(data));
  };

  doRegister = (e: SyntheticEvent<*>) => {
    e.preventDefault();
    const data = this.loginData();
    const xhr = new XMLHttpRequest();
    xhr.open("POST", `${this.props.api}/api/register`);
    xhr.setRequestHeader("Content-type", "application/json");
    xhr.onreadystatechange = () => {
      if (xhr.readyState === 4) {
        if (xhr.status === 200) {
          this.setState({
            message: `Registration succeeded, please log in`,
            loggedIn: false,
          });
        } else {
          const loginResponse = JSON.parse(xhr.responseText);
          this.setState({
            message: `Registration failed with ${loginResponse.message}`,
            loggedIn: false,
          });
        }
      }
    };
    xhr.send(JSON.stringify(data));
  };

  loginData = (): LoginRegister => ({
    email: this.state.email,
    password: this.state.password,
  });

  render() {
    const emailChange = (e: SyntheticKeyboardEvent<HTMLInputElement>) =>
      this.setState({ email: e.currentTarget.value || "" });
    const pwChange = (e: SyntheticKeyboardEvent<HTMLInputElement>) =>
      this.setState({ password: e.currentTarget.value || "" });
    const form = !this.state.loggedIn ? (
      <form>
        <input type="email" value={this.state.email} onChange={emailChange} />
        <input
          type="password"
          value={this.state.password}
          onChange={pwChange}
        />
        <button type="submit" onClick={this.doLogin}>
          Login
        </button>
        <button type="submit" onClick={this.doRegister}>
          Register
        </button>
      </form>
    ) : (
      <div />
    );
    return (
      <div>
        {form}
        <div>{this.state.message}</div>
      </div>
    );
  }
}
