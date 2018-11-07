// @flow

import * as React from "react";
import ImagePreview from "./ImagePreview";
import ImageFrame from "./ImageFrame";
import Action from "./Action";
import styles from "./Uploader.css";

type State = {|
  selected: number,
|};

export default class Uploader extends React.Component<
  { files: Array<File> },
  State,
> {
  state: State = {
    selected: 0,
  };

  next = () => {
    this.setState(state => {
      if (this.props.files.length > state.selected + 1) {
        return { selected: state.selected + 1 };
      }
      return state;
    });
  };

  previous = () => {
    this.setState(state => {
      if (state.selected > 0) {
        return { selected: state.selected - 1 };
      }
      return state;
    });
  };

  select = (idx: number) => () => {
    this.setState({ selected: idx });
  };

  render() {
    const { files } = this.props;
    return (
      <React.Fragment>
        <div className={styles.preview}>
          {files.map((file, idx) => (
            <ImageFrame key={file.name}>
              {idx === this.state.selected ? (
                <ImagePreview file={file} selected />
              ) : (
                <ImagePreview file={file} onClick={this.select(idx)} />
              )}
            </ImageFrame>
          ))}
        </div>
        <div className={styles.pane}>
          <div className={styles.action}>
            <Action label="Next" keybinding="ArrowRight" do={this.next} />
          </div>
          <div className={styles.action}>
            <Action
              label="Previous"
              keybinding="ArrowLeft"
              do={this.previous}
            />
          </div>
        </div>
        <div className={styles.previewFrame}>
          {this.props.files.length > 0 && (
            <ImagePreview file={this.props.files[this.state.selected]} />
          )}
        </div>
      </React.Fragment>
    );
  }
}
