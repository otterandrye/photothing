// @flow

// Use a separate client.js file for the galler to avoid shipping all the app
// dependencies to gallery viewers

import * as React from "react";
import ReactDom from "react-dom";
import Page from "./Page";

if (document.documentElement) {
  ReactDom.hydrate(<Page {...window.MANIFEST} />, document.documentElement);
}
