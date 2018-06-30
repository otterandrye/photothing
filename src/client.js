import * as React from "react";
import ReactDom from "react-dom";
import App from "./app";

/* eslint no-alert: 0 */
/* eslint flowtype/require-valid-file-annotation: 0 */
// :-P sorry
// given it's PoC code not worth making flow happy on document.getElement calls

console.log("Hello World, from the client side JS");

// upload test nonsense here
// https://devcenter.heroku.com/articles/s3-upload-node

function uploadFile(file, signedRequest, url) {
  console.log(`file upload started to ${url}`);
  const xhr = new XMLHttpRequest();
  xhr.open("PUT", signedRequest);
  xhr.onreadystatechange = () => {
    if (xhr.readyState === 4) {
      if (xhr.status === 200) {
        // NB: this points to the raw S3 bucket url e.g.
        // https://photothing-dev.s3.us-east-2.amazonaws.com/troll.jpg
        // which we eventually want to route through cloudfront instead
        // NB: only the dev bucket is set to serve its contents via http(s)
        document.getElementById("preview").src = url;
      } else {
        alert("Could not upload file.");
      }
    }
  };
  xhr.send(file);
}

function getSignedRequest(file) {
  console.log(`getting signed request for ${file.name}`);
  const xhr = new XMLHttpRequest();
  xhr.open("GET", `/sign-s3?file-name=${file.name}&file-type=${file.type}`);
  xhr.onreadystatechange = () => {
    if (xhr.readyState === 4) {
      if (xhr.status === 200) {
        const response = JSON.parse(xhr.responseText);
        uploadFile(file, response.signedRequest, response.url);
      } else {
        alert("Could not get signed URL.");
      }
    }
  };
  xhr.send();
}

(() => {
  (document.getElementById("file-input") || {}).onchange = () => {
    console.log("file input onchange fired");
    const input = document.getElementById("file-input");
    const files = input && input.files;
    const file = files && files[0];
    if (file == null) {
      alert("No file selected.");
    }
    getSignedRequest(file);
  };
})();

// end of upload test nonsense
if (!document.getElementById("file-input")) {
  ReactDom.hydrate(
    <App scripts={window.SCRIPTS} styles={window.STYLES} />,
    document.documentElement,
  );
}
