// @flow
/* eslint-disable no-restricted-globals */

import * as React from "react";
import { type AuthContext } from "./api";

type Props = {|
  api: string,
  authContext: AuthContext | null,
  authenticate: AuthContext => any,
  logout: () => any,
|};

type State = {|
  email: string,
  password: string,
  message: string | null,
  finishPwReset: boolean,
|};

type LoginRegister = {|
  email: string,
  password: string,
|};

type UserCredentials = {|
  email: string,
  header: string,
  pt_auth: string,
|};

export default class Login extends React.Component<Props, State> {
  state: State = {
    email: "",
    password: "",
    message: null,
    finishPwReset: false,
  };

  componentDidMount() {
    const params = new URLSearchParams(location.search.slice(1));
    const emailForPwReset = params.get("email");
    if (emailForPwReset) {
      console.log("Setting email from PW reset url param");
      this.setState({ email: emailForPwReset, finishPwReset: true });
    }
  }

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
          this.setState({ message: null });
          const login: UserCredentials = JSON.parse(xhr.responseText);
          this.props.authenticate({
            header: login.header,
            token: login.pt_auth,
            email: login.email,
          });
        } else {
          this.setState({
            message: `Login failed with status=${xhr.status}`,
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
          });
        } else {
          const loginResponse = JSON.parse(xhr.responseText);
          this.setState({
            message: `Registration failed with ${loginResponse.message}`,
          });
        }
      }
    };
    xhr.send(JSON.stringify(data));
  };

  startPwReset = (e: SyntheticEvent<*>) => {
    e.preventDefault();
    const email = encodeURIComponent(this.state.email);
    const xhr = new XMLHttpRequest();
    xhr.open("POST", `${this.props.api}/api/reset_password/${email}`);
    xhr.setRequestHeader("Content-type", "application/json");
    xhr.onreadystatechange = () => {
      if (xhr.readyState === 4) {
        if (xhr.status === 200) {
          this.setState({
            message: "Initiated password reset, check your email",
            password: "",
          });
        } else {
          this.setState({
            message: "Failed to start the password reset process, check logs",
          });
        }
      }
    };
    xhr.send("{}");
  };

  finishPwReset = (e: SyntheticEvent<*>) => {
    e.preventDefault();
    const params = new URLSearchParams(location.search.slice(1));
    const id = params.get("id") || "";
    const email = params.get("email") || "";
    if (!id || !email) {
      return;
    }
    this.setState({ email }, () => this.sendPwReset(id));
  };

  sendPwReset = (id: string) => {
    const data = this.loginData();
    const xhr = new XMLHttpRequest();
    xhr.open("PUT", `${this.props.api}/api/reset_password/${id}`);
    xhr.setRequestHeader("Content-type", "application/json");
    xhr.onreadystatechange = () => {
      if (xhr.readyState === 4) {
        const res = JSON.parse(xhr.responseText);
        if (xhr.status === 200) {
          this.setState({
            message: `Reset request: '${res.reset}' please log in`,
            finishPwReset: false,
          });
        } else {
          this.setState({
            message: `Reset request failed with ${res.message}`,
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

    let buttons;
    if (this.state.finishPwReset) {
      buttons = (
        <button type="submit" onClick={this.finishPwReset}>
          Finish Password Reset
        </button>
      );
    } else if (this.props.authContext) {
      buttons = (
        <button type="submit" onClick={this.props.logout}>
          Log out
        </button>
      );
    } else {
      buttons = [
        <button key="login" type="submit" onClick={this.doLogin}>
          Login
        </button>,
        <button key="reg" type="submit" onClick={this.doRegister}>
          Register
        </button>,
        <button key="reset" type="submit" onClick={this.startPwReset}>
          Start Password Reset
        </button>,
      ];
    }

    const form = !this.props.authContext ? (
      <form>
        <input type="email" value={this.state.email} onChange={emailChange} />
        <input
          type="password"
          value={this.state.password}
          onChange={pwChange}
        />
        {buttons}
      </form>
    ) : (
      <div>{buttons}</div>
    );
    return (
      <div>
        {form}
        <div>
          {this.state.message ||
            (this.props.authContext
              ? `Welcome, ${this.props.authContext.email}`
              : null)}
        </div>
      </div>
    );
  }
}
