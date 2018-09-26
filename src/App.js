// @flow

import * as React from "react";
import styles from "./App.css";

import type { Route } from "./routes";
import { ApiProvider } from "./Api";
import Library from "./Library";
import Login from "./auth/Login";
import SignUp from "./auth/SignUp";
import ForgotPassword from "./auth/ForgotPassword";
import ResetPassword from "./auth/ResetPassword";
import Album from "./Album";
import Menu from "./Menu";

type AuthContext = {|
  email: string,
  header: string,
  token: string,
|};

type State = {|
  auth: AuthContext | null,
|};

type Props = {|
  +api: string,
  +navigate: Route => void,
  +route: Route,
|};

export default class App extends React.Component<Props, State> {
  state: State = {
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
    } else if (this.props.route.page === "LIBRARY") {
      localStorage.clear();
      this.props.navigate({ page: "LOGIN" });
    }
  }

  authenticate = (authContext: AuthContext) => {
    localStorage.setItem("header", authContext.header);
    localStorage.setItem("token", authContext.token);
    localStorage.setItem("email", authContext.email);
    this.setState({ auth: authContext });
    this.props.navigate({ page: "LIBRARY" });
  };

  logout = () => {
    localStorage.clear();
    this.setState({ auth: null });
    this.props.navigate({ page: "LOGIN" });
  };

  render() {
    const { route } = this.props;
    return (
      <ApiProvider
        value={{
          host: this.props.api,
          headers: this.state.auth
            ? {
                [this.state.auth.header]: this.state.auth.token,
              }
            : {},
        }}
      >
        <Menu />
        <div className={styles.content}>
          {this.state.auth && (
            <React.Fragment>
              Welcome, {this.state.auth.email}.
              <button type="button" onClick={this.logout}>
                Log out
              </button>
            </React.Fragment>
          )}
          {route.page === "LOGIN" && (
            <Login
              api={this.props.api}
              authenticate={this.authenticate}
              navigate={this.props.navigate}
            />
          )}
          {route.page === "SIGNUP" && (
            <SignUp api={this.props.api} navigate={this.props.navigate} />
          )}
          {route.page === "FORGOT_PASSWORD" && (
            <ForgotPassword api={this.props.api} />
          )}
          {route.page === "RESET_PASSWORD" && (
            <ResetPassword
              api={this.props.api}
              email={route.email}
              id={route.id}
              navigate={this.props.navigate}
            />
          )}
          {this.state.auth && route.page === "LIBRARY" && <Library />}
          {route.page === "ALBUM" && (
            <Album id={route.albumId} photoId={route.photoId} />
          )}
        </div>
      </ApiProvider>
    );
  }
}
