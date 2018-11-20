// @flow

import express from "express";
import fetch from "node-fetch";
import * as React from "react";
import ReactDomServer from "react-dom/server";

import Page from "./Page";
import GalleryPage from "./public_gallery/Page";
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
server.use("/view/static", express.static("dist"));

const serverSideRender = res => component => {
  res.write("<!doctype html><html>");
  const stream = ReactDomServer.renderToNodeStream(component);
  stream.pipe(
    res,
    { end: false },
  );
  stream.on("end", () => {
    res.end("</html>");
  });
};

server.get("/view/:hash/", (req, res) => {
  const { hash } = req.params;
  fetch(`${API_SERVER}/api/published/${hash}`)
    .then(r => {
      if (r.ok) {
        r.json().then(json => {
          serverSideRender(res)(
            <GalleryPage
              scripts={["static/gallery-client.js"]}
              styles={["static/main.css"]}
              api={API_SERVER}
              album={json}
              hash={hash}
            />,
          );
        });
      } else {
        res.end(`sorry, gallery at '${req.params.hash}' was not found`);
      }
    })
    .catch(err => {
      console.error(err);
      res.end("Something went wrong on our end, we're looking into it");
    });
});

server.get("*", (req, res) => {
  serverSideRender(res)(
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
        modals: [],
      }}
    />,
  );
});

server.listen(port, () => console.log(`Chroma listening on port ${port}!`));
