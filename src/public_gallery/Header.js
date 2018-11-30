// @flow

import * as React from "react";
import css from "./Header.css";

type Props = {|
  text: string,
|};

export default (props: Props) => (
  <div className={css.header}>
    <div className={css.logoBox}>
      <span className={css.logo}>{"\u23e3"}</span>
      <span className={css.name}>chroma</span>
    </div>
    <div className={css.textBox}>
      <h1 className={css.text}>{props.text}</h1>
    </div>
  </div>
);
