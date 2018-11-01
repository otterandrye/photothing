// @flow

import express from "express";
import * as React from "react";
import ReactDomServer from "react-dom/server";

import Page from "./Page";
import { parseRoute } from "./routes";

const port = process.env.PORT || 3000;
const server = express();

// heroku handles ssl termination, check headers for the original scheme
if (process.env.NODE_ENV === "production") {
  server.use((req, res, next) => {
    res.setHeader(
      "Strict-Transport-Security",
      "max-age=8640000; includeSubDomains",
    );
    if (req.headers["x-forwarded-proto"] !== "https") {
      return res.redirect(301, `https://${req.hostname}${req.url}`);
    }
    return next();
  });
}

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
express.static.mime.define({ "application/wasm": ["wasm"] });
server.use("/static", express.static("dist"));

server.get("*", (req, res) => {
  res.write("<!doctype html><html>");
  const stream = ReactDomServer.renderToNodeStream(
    <Page
      scripts={["static/client.js"]}
      styles={["static/main.css"]}
      state={{
        api: {
          host: API_SERVER,
          headers: {},
        },
        auth: null,
        route: parseRoute(req.url),
      }}
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

server.listen(port, () =>
  console.log(`Photo thing listening on port ${port}!`),
);
