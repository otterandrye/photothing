// @flow

import * as React from "react";
import { connect } from "react-redux";
import { getPath, type Route } from "./routes";

type Props = {|
  route: Route,
|};

class History extends React.Component<Props> {
  componentDidMount() {
    // eslint-disable-next-line no-restricted-globals
    history.replaceState(this.props.route, "", getPath(this.props.route));
  }

  componentDidUpdate(prevProps: Props) {
    // eslint-disable-next-line no-restricted-globals
    history.pushState(this.props.route, "", getPath(this.props.route));
    if (prevProps.route.page !== this.props.route.page) {
      window.scrollTo(0, 0);
    }
  }

  render() {
    return null;
  }
}

export default connect(state => ({ route: state.route }))(History);
