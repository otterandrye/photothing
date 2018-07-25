import * as React from "react";
import ReactDom from "react-dom";
import Page from "./Page";

/* eslint flowtype/require-valid-file-annotation: 0 */
// :-P sorry
// given it's PoC code not worth making flow happy on document.getElement calls

console.log("Hello World, from the client side JS");

if (!document.getElementById("file-input")) {
  ReactDom.hydrate(<Page {...window.MANIFEST} />, document.documentElement);
}
