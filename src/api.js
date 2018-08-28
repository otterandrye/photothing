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

type Props = {|
  +method: IdempotentHttpVerb,
  +path: string,
  +children: RequestState => React.Node,
|};

const Api = (props: Props) => (
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

export default Api;
