// @flow

import * as React from "react";
import { connect } from "react-redux";

type Props = {|
  +api: string,
|};

type State = {|
  email: string,
  success: boolean,
  message: string,
|};

class ForgotPassword extends React.Component<Props, State> {
  state: State = { email: "", success: false, message: "Enter your email" };

  setEmail = (evt: SyntheticInputEvent<*>) =>
    this.setState({ email: evt.target.value });

  submit = (evt: SyntheticEvent<*>) => {
    evt.preventDefault();
    fetch(
      `${this.props.api}/api/reset_password/${encodeURIComponent(
        this.state.email,
      )}`,
      {
        method: "POST",
      },
    )
      .then(res => {
        if (res.ok) {
          this.setState({
            success: true,
            message: "Success! Please check your email.",
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
              type="email"
              value={this.state.email}
              onChange={this.setEmail}
              autoComplete="email"
              required
            />
            <button type="submit">Reset Password</button>{" "}
          </form>
        )}
      </React.Fragment>
    );
  }
}

export default connect(state => ({ api: state.api.host }))(ForgotPassword);
