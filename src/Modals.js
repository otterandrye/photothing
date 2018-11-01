// @flow

import * as React from "react";
import { connect } from "react-redux";
import { popModal } from "./State";
import css from "./Modals.css";

type Props = {|
  +children: null | React.Node,
  +close: () => void,
|};

const Modals = ({ children, close }: Props) =>
  children && (
    <React.Fragment>
      <div className={css.shade} onClick={close} />
      <div className={css.modal}>{children}</div>
    </React.Fragment>
  );

export default connect(
  state => ({
    children: state.modals.length > 0 ? state.modals[0] : null,
  }),
  { close: popModal },
)(Modals);
