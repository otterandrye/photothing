// @flow

import * as React from "react";
import styles from "./Action.css";

type Props = {|
  label: string,
  keybinding: string,
  do: () => void,
|};

const translate = key => {
  switch (key) {
    case "ArrowRight":
      return "\u2192";
    case "ArrowLeft":
      return "\u2190";
    default:
      return key;
  }
};

class Action extends React.Component<Props, {| highlight: boolean |}> {
  state = { highlight: false };

  componentDidMount() {
    document.addEventListener("keydown", this.downListener, false);
    document.addEventListener("keyup", this.upListener, false);
    document.addEventListener("keypress", this.bindingListener, false);
  }

  componentWillUnmount() {
    document.removeEventListener("keyup", this.upListener, false);
    document.removeEventListener("keydown", this.downListener, false);
    document.removeEventListener("keypress", this.bindingListener, false);
  }

  downListener = (evt: KeyboardEvent) => {
    if (evt.key === "Alt") {
      this.setState({ highlight: true });
    }
  };

  upListener = (evt: KeyboardEvent) => {
    if (evt.key === "Alt") {
      this.setState({ highlight: false });
    }
  };

  bindingListener = (evt: KeyboardEvent) => {
    if (evt.key === this.props.keybinding) {
      this.props.do();
    }
  };

  render() {
    return (
      <React.Fragment>
        <span
          className={`${styles.indicator}${
            this.state.highlight ? ` ${styles.revealed}` : ""
          }`}
          alt={`This action can be triggered with the key command: ${translate(
            this.props.keybinding,
          )}`}
        >
          {translate(this.props.keybinding)}
        </span>
        <span className={styles.label}>{this.props.label}</span>
      </React.Fragment>
    );
  }
}

export default Action;
