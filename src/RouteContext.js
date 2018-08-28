// @flow

import * as React from "react";
import type { Route } from "./routes";

type RouteContext = {|
  route: Route,
  navigate: Route => void,
|};

const { Provider, Consumer } = React.createContext(
  ({
    route: {
      page: "404", // This is a weird default. Maybe we shouldn't use contexts here?
    },
    navigate: () => {},
  }: RouteContext),
);

export const RouteProvider = Provider;
export const RouteConsumer = Consumer;

// The public API here...

// <RouteProvider value={}></RouteProvider>
// <RouteConsumer>{({route, navigate}) => {}}</RouteConsumer>

// The future public api...

//  <Match k={v}></Match> (if v instanceof RegExp test, if v instanceof Function call w/ key value)
//  <Link to={$Shape<Route>}></Link>
//  And then a History updater-component to keep the url in sync.
