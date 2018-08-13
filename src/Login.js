// @flow
/* eslint-disable no-restricted-globals */

import * as React from "react";

type Props = {|
  api: string,
|};

type State = {|
  email: string,
  password: string,
  message: string | null,
  loggedIn: boolean,
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
    loggedIn: false,
    finishPwReset: false,
  };

  componentDidMount() {
    const email = localStorage.getItem("email");
    if (email) {
      console.log("logging in from localStorage");
      this.login(email);
    }
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
          this.login(data.email);
          // _sigh_ maybe time for some lightweight state to stash these
          const login: UserCredentials = JSON.parse(xhr.responseText);
          localStorage.setItem("header", login.header);
          localStorage.setItem("token", login.pt_auth);
          localStorage.setItem("email", login.email);
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

  login = (email: string) => {
    this.setState({
      message: `Welcome, ${email}`,
      loggedIn: true,
    });
  };

  doLogout = () => {
    localStorage.clear();
    window.location.reload();
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
            loggedIn: false,
            password: "",
          });
        } else {
          this.setState({
            message: "Failed to start the password reset process, check logs",
            loggedIn: false,
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
            loggedIn: false,
            finishPwReset: false,
          });
        } else {
          this.setState({
            message: `Reset request failed with ${res.message}`,
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

    let buttons;
    if (this.state.finishPwReset) {
      buttons = (
        <button type="submit" onClick={this.finishPwReset}>
          Finish Password Reset
        </button>
      );
    } else if (this.state.loggedIn) {
      buttons = (
        <button type="submit" onClick={this.doLogout}>
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

    const form = !this.state.loggedIn ? (
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
        <div>{this.state.message}</div>
      </div>
    );
  }
}
