// @flow

import express from "express";
import aws from "aws-sdk";

import template from "./template";
import upload from "./upload";

const port = process.env.PORT || 3000;
const app = express();

function env(key): string {
  const value = process.env[key];
  if (typeof value !== "string") {
    console.error(`missing environment variable ${key}`);
    process.exit(1);
    return ""; // unreachable
  }
  return value;
}

app.set("x-powered-by", false);
app.get("/", (req, res) =>
  res.send(
    template({
      title: "Photo Thing!",
      body: "Hello World with HTML",
    }),
  ),
);
app.use("/static", express.static("dist"));

// signed upload tutorial code starts here
const S3_REGION = env("S3_REGION");
const S3_BUCKET_NAME = env("S3_BUCKET_NAME");
aws.config.region = S3_REGION;

console.log(`s3 bucket: ${S3_BUCKET_NAME}`);

app.get("/upload", (_, res) => res.send(upload(S3_BUCKET_NAME)));

app.get("/sign-s3", (req, res) => {
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

app.listen(port, () => console.log(`Photo thing listening on port ${port}!`));
