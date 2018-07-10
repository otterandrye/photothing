// @flow

import * as React from "react";
import css from "./ImageFrame.css";

export default ({ children }: {| +children: React.Node |}) => (
  <div className={css.frame}>{children}</div>
);
