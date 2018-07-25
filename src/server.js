// @flow

import express from "express";
import * as React from "react";
import ReactDomServer from "react-dom/server";

import Page from "./Page";

const port = process.env.PORT || 3000;
const server = express();

function env(key): string {
  const value = process.env[key];
  if (typeof value !== "string") {
    console.error(`missing environment variable ${key}`);
    process.exit(1);
    return ""; // unreachable
  }
  return value;
}

const API_SERVER = env("API_SERVER");

server.set("x-powered-by", false);
server.get("/", (req, res) => {
  res.write("<!doctype html><html>");
  const stream = ReactDomServer.renderToNodeStream(
    <Page
      scripts={["static/client.js"]}
      styles={["static/main.css"]}
      api={API_SERVER}
    />,
  );
  stream.pipe(
    res,
    { end: false },
  );
  stream.on("end", () => {
    res.end("</html>");
  });
});

express.static.mime.define({ "application/wasm": ["wasm"] });
server.use("/static", express.static("dist"));

server.listen(port, () =>
  console.log(`Photo thing listening on port ${port}!`),
);
