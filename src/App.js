// @flow

import * as React from "react";
import { connect } from "react-redux";
import styles from "./App.css";

import type { Route } from "./routes";
import Library from "./Library";
import Login from "./auth/Login";
import SignUp from "./auth/SignUp";
import ForgotPassword from "./auth/ForgotPassword";
import ResetPassword from "./auth/ResetPassword";
import Album from "./Album";
import Albums from "./Albums";
import Menu from "./Menu";
import Modals from "./Modals";

type Props = {|
  route: Route,
  isAuthenticated: boolean,
|};

const App = ({ route, isAuthenticated }: Props) => (
  <React.Fragment>
    {isAuthenticated && (
      <React.Fragment>
        <Menu />
        <div className={styles.content}>
          {route.page === "LIBRARY" && <Library />}
          {route.page === "ALBUMS" && <Albums />}
          {route.page === "ALBUM" && (
            <Album id={route.albumId} photoId={route.photoId} />
          )}
        </div>
      </React.Fragment>
    )}
    {route.page === "LOGIN" && <Login />}
    {route.page === "SIGNUP" && <SignUp />}
    {route.page === "FORGOT_PASSWORD" && <ForgotPassword />}
    {route.page === "RESET_PASSWORD" && (
      <ResetPassword email={route.email} id={route.id} />
    )}
    <Modals />
  </React.Fragment>
);

export default connect(({ route, auth }) => ({
  route,
  isAuthenticated: auth !== null,
}))(App);
