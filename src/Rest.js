// @flow

import * as React from "react";

export type IdempotentHttpVerb = "GET" | "PUT" | "DELETE";

export type RequestState<T, E = string> = {|
  data: T | null,
  loading: boolean,
  error: E | null,
|};

type Props<T> = {|
  +method: IdempotentHttpVerb,
  +host: string,
  +path: string,
  +headers: { [string]: string },
  +children: (RequestState<T>) => React.Node,
|};

export default class Rest<T> extends React.Component<
  Props<T>,
  RequestState<T>,
> {
  static defaultProps = {
    method: "GET",
  };

  state: RequestState<T> = { data: null, error: null, loading: true };

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
