// @flow

import * as React from "react";

export type IdempotentHttpVerb = "GET" | "PUT" | "DELETE";

export type RequestState = {|
  data: any,
  loading: boolean,
  error: any,
|};

type Props = {|
  +method: IdempotentHttpVerb,
  +host: string,
  +path: string,
  +headers: { [string]: string },
  +children: RequestState => React.Node,
|};

export default class Rest extends React.Component<Props, RequestState> {
  static defaultProps = {
    method: "GET",
  };

  state = { data: null, error: null, loading: true };

  componentDidMount() {
    fetch(`${this.props.host}${this.props.path}`, {
      method: this.props.method,
      headers: this.props.headers,
    }).then(
      res => res.json().then(data => this.setState({ loading: false, data })),
      err => err.json().then(error => this.setState({ loading: false, error })),
    );
  }

  render() {
    return this.props.children(this.state);
  }
}
