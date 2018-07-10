// @flow

import express from "express";
import aws from "aws-sdk";
import * as React from "react";
import ReactDomServer from "react-dom/server";

import upload from "./upload";
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

server.set("x-powered-by", false);
server.get("/", (req, res) => {
  res.write("<!doctype html><html>");
  const stream = ReactDomServer.renderToNodeStream(
    <Page scripts={["static/client.js"]} styles={["static/main.css"]} />,
  );
  stream.pipe(
    res,
    { end: false },
  );
  stream.on("end", () => {
    res.end("</html>");
  });
});
server.use("/static", express.static("dist"));

// signed upload tutorial code starts here
const S3_REGION = env("S3_REGION");
const S3_BUCKET_NAME = env("S3_BUCKET_NAME");
aws.config.region = S3_REGION;

server.get("/upload", (_, res) => res.send(upload(S3_BUCKET_NAME)));

server.get("/sign-s3", (req, res) => {
  const s3 = new aws.S3();
  const fileName = req.query["file-name"];
  const fileType = req.query["file-type"];
  const s3Params = {
    Bucket: S3_BUCKET_NAME,
    Key: fileName,
    Expires: 60,
    ContentType: fileType,
    ACL: "public-read",
  };

  s3.getSignedUrl("putObject", s3Params, (err, data) => {
    if (err) {
      console.log(err);
      return res.end();
    }
    const returnData = {
      signedRequest: data,
      url: `https://${S3_BUCKET_NAME}.s3.${S3_REGION}.amazonaws.com/${fileName}`,
    };
    res.write(JSON.stringify(returnData));
    return res.end();
  });
});

// and ends here

server.listen(port, () =>
  console.log(`Photo thing listening on port ${port}!`),
);
