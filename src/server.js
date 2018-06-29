// @flow

import express from "express";
import template from "./template";

const port = process.env.PORT || 3000;
const app = express();

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
app.listen(port, () => console.log(`Photo thing listening on port ${port}!`));
