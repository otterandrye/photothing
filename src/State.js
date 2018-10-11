// @flow
/* eslint-disable import/prefer-default-export, no-unused-vars */

import { type Route } from "./routes";

export type Auth = {|
  email: string,
  header: string,
  token: string,
|};

type Navigation = {|
  type: "NAVIGATION",
  to: Route,
|};

type Authentication = {|
  type: "AUTHENTICATION",
  credentials: Auth,
|};

type Logout = {|
  type: "LOGOUT",
|};

export const authenticate = (auth: Auth) => ({
  type: "AUTHENTICATION",
  credentials: auth,
});

export const navigate = (route: Route) => ({
  type: "NAVIGATION",
  to: route,
});

export const logout = () => ({ type: "LOGOUT" });

export type Action = Navigation | Authentication | Logout;

type ApiState = {|
  host: string,
  headers: { [string]: string },
|};

export type State = {|
  route: Route,
  auth: Auth | null,
  api: ApiState,
|};

export const reducer = (state: State, action: Action) => {
  if (action.type === "NAVIGATION") {
    return { ...state, route: action.to };
  }
  if (action.type === "AUTHENTICATION") {
    const newState = {
      ...state,
      auth: action.credentials,
      api: {
        ...state.api,
        headers: {
          ...state.api.headers,
          [action.credentials.header]: action.credentials.token,
        },
      },
    };
    if (newState.route.page === "LOGIN") {
      newState.route = { page: "LIBRARY" };
    }
    return newState;
  }
  if (action.type === "LOGOUT") {
    return { ...state, route: { page: "LOGIN" }, auth: null };
  }
  return state;
};
