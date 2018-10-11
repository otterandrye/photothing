// @flow

import { connect } from "react-redux";
import Rest, { type RequestState } from "./Rest";

export type { RequestState };

export default connect(state => ({ ...state.api }))(Rest);
