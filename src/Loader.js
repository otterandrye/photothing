// @flow

import * as React from "react";
import css from "./Loader.css";

// $FlowFixMe: https://github.com/facebook/flow/issues/6103
export default React.forwardRef((_, ref) => (
  <div className={css.loader} ref={ref} />
));
