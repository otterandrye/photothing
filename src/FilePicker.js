// @flow

import * as React from "react";

import css from "./FilePicker.css";

type Props = {|
  +accept: Array<string>,
  +onSelect: (Array<File>) => void,
  +children: (() => void) => React.Node,
|};

type State = {|
  files: Array<File>,
|};

export default class FilePicker extends React.Component<Props, State> {
  constructor(props: Props) {
    super(props);
    this.inputRef = React.createRef();
  }

  open = () => {
    if (this.inputRef.current) {
      this.inputRef.current.click();
    }
  };

  handler = (evt: SyntheticEvent<HTMLInputElement>) => {
    this.props.onSelect(Array.from(evt.currentTarget.files));
  };

  inputRef: { current: null | HTMLInputElement };

  render() {
    return (
      <React.Fragment>
        <input
          className={css.hidden}
          type="file"
          multiple
          accept={this.props.accept.join(", ")}
          onChange={this.handler}
          ref={this.inputRef}
        />
        {this.props.children(this.open)}
      </React.Fragment>
    );
  }
}
