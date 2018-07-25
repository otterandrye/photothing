import * as React from "react";
import ReactDom from "react-dom";
import Page from "./Page";

/* eslint no-alert: 0 */
/* eslint flowtype/require-valid-file-annotation: 0 */
// :-P sorry
// given it's PoC code not worth making flow happy on document.getElement calls

console.log("Hello World, from the client side JS");

// upload test nonsense here
// https://devcenter.heroku.com/articles/s3-upload-node

function uploadFile(file, uploadResponse) {
  console.log(`file upload started to ${uploadResponse.url}`);
  const xhr = new XMLHttpRequest();
  xhr.open("PUT", uploadResponse.url);
  xhr.onreadystatechange = () => {
    if (xhr.readyState === 4) {
      if (xhr.status === 200) {
        document.getElementById("preview").src = uploadResponse.get_url;
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
  xhr.open("POST", "http://localhost:8000/api/upload");
  xhr.setRequestHeader("Content-Type", "application/json;charset=UTF-8");
  xhr.send(JSON.stringify({ filename: file.name, file_type: file.type }));
  xhr.onreadystatechange = () => {
    if (xhr.readyState === 4) {
      if (xhr.status === 200) {
        const response = JSON.parse(xhr.responseText);
        console.log(`got json response from rust: ${xhr.responseText}`);
        uploadFile(file, response);
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
  ReactDom.hydrate(<Page {...window.MANIFEST} />, document.documentElement);
}
