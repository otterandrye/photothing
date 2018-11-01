// @flow
/* eslint-disable import/prefer-default-export, no-unused-vars */

import * as React from "react";
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

type PushModal = {|
  type: "PUSH_MODAL",
  modal: React.Node,
|};

type PopModal = {|
  type: "POP_MODAL",
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

export const pushModal = (modal: React.Node) => ({ type: "PUSH_MODAL", modal });

export const popModal = () => ({ type: "POP_MODAL" });

export type Action =
  | Navigation
  | Authentication
  | Logout
  | PushModal
  | PopModal;

type ApiState = {|
  host: string,
  headers: { [string]: string },
|};

export type State = {|
  route: Route,
  auth: Auth | null,
  api: ApiState,
  modals: Array<React.Node>,
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
  if (action.type === "PUSH_MODAL") {
    return { ...state, modals: [action.modal, ...state.modals] };
  }
  if (action.type === "POP_MODAL") {
    const newStack = [...state.modals];
    newStack.shift();
    return { ...state, modals: newStack };
  }
  return state;
};
