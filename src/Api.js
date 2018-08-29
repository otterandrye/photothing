// @flow

import * as React from "react";
import Rest, { type IdempotentHttpVerb, type RequestState } from "./Rest";

export type ApiContext = {|
  host: string,
  headers: { [string]: string },
|};

const { Provider, Consumer } = React.createContext(
  ({ host: "", headers: {} }: ApiContext),
);
export const ApiProvider = Provider;
export const ApiConsumer = Consumer;

type Props<T> = {|
  +method: IdempotentHttpVerb,
  +path: string,
  +children: (RequestState<T>) => React.Node,
|};

const Api = <T>(props: Props<T>) => (
  <Consumer>
    {context => (
      <Rest
        method={props.method}
        path={props.path}
        host={context.host}
        headers={context.headers}
      >
        {props.children}
      </Rest>
    )}
  </Consumer>
);

Api.defaultProps = { method: "GET" };

export type { RequestState };

export default Api;
