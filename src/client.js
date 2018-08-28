// @flow

import * as React from "react";
import ReactDom from "react-dom";
import Page from "./Page";

if (document.documentElement) {
  ReactDom.hydrate(<Page {...window.MANIFEST} />, document.documentElement);
}
