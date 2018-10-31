// @flow

import * as React from "react";
import { connect } from "react-redux";
import { authenticate, logout, type Auth } from "./State";

type Props = {|
  +status: Auth | null,
  +authenticate: Auth => void,
  +logout: () => void,
|};

class Authenticator extends React.Component<Props> {
  componentDidMount() {
    const authContext = {
      email: localStorage.getItem("email"),
      header: localStorage.getItem("header"),
      token: localStorage.getItem("token"),
    };
    if (authContext.email && authContext.header && authContext.token) {
      // $FlowFixMe: Flow can't handle this refinement
      this.props.authenticate(authContext);
    } else {
      this.props.logout();
    }
  }

  componentDidUpdate() {
    const { status } = this.props;
    if (status) {
      localStorage.setItem("header", status.header);
      localStorage.setItem("token", status.token);
      localStorage.setItem("email", status.email);
    } else {
      localStorage.clear();
      this.props.logout();
    }
  }

  render() {
    return null;
  }
}

export default connect(
  state => ({ status: state.auth }),
  { authenticate, logout },
)(Authenticator);
