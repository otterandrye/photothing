// @flow

import * as React from "react";
import { connect } from "react-redux";
import ImagePreview from "./ImagePreview";
import ImageFrame from "./ImageFrame";
import Action from "./Action";
import styles from "./Uploader.css";

type FileState = {|
  status: "INITIAL" | "QUEUED" | "UPLOADING" | "UPLOADED",
  progress: number,
|};

type State = {|
  selected: File,
  statuses: Map<File, FileState>,
|};

const uploadFile = (
  api: string,
  headers: { [string]: string },
  file: File,
  mark: number => void,
) => {
  mark(0);
  fetch(`${api}/api/upload`, {
    method: "POST",
    headers,
    body: JSON.stringify({
      filename: file.name,
      file_type: file.type,
    }),
  })
    .then(res => res.json())
    .then(pendingUpload => pendingUpload.upload.url)
    .then(
      signedRequest => {
        const xhr = new XMLHttpRequest();
        xhr.upload.addEventListener(
          "progress",
          (evt: ProgressEvent) => {
            mark(evt.loaded / evt.total);
          },
          false,
        );
        xhr.open("PUT", signedRequest);
        xhr.onreadystatechange = () => {
          if (xhr.readyState === 4) {
            if (xhr.status === 200) {
              mark(1);
            }
          }
        };
        xhr.send(file);
      },
      e => {
        console.error(`Upload failed: '${e}'`);
      },
    );
};

class Uploader extends React.Component<
  {| +files: Array<File>, +api: string, +headers: { [string]: string } |},
  State,
> {
  state: State = {
    selected: this.props.files[0],
    // eslint-disable-next-line react/no-unused-state
    statuses: new Map(
      this.props.files.map(file => [file, { status: "INITIAL", progress: 0 }]),
    ),
  };

  next = () => {
    this.setState(state => {
      const idx = this.props.files.indexOf(this.state.selected);
      if (this.props.files.length > idx + 1) {
        return { selected: this.props.files[idx + 1] };
      }
      return state;
    });
  };

  previous = () => {
    this.setState(state => {
      const idx = this.props.files.indexOf(this.state.selected);
      if (idx > 0) {
        return { selected: this.props.files[idx - 1] };
      }
      return state;
    });
  };

  select = (idx: number) => () => {
    this.setState({ selected: this.props.files[idx] });
  };

  marker = file => percentage => {
    this.setState(state => {
      const statuses = new Map(state.statuses.entries());

      let status = "UPLOADING";
      if (percentage === 0) {
        status = "QUEUED";
      } else if (percentage === 1) {
        status = "UPLOADED";
      }

      statuses.set(file, {
        status,
        progress: percentage,
      });

      return {
        statuses,
      };
    });
  };

  upload = () => {
    uploadFile(
      this.props.api,
      this.props.headers,
      this.state.selected,
      this.marker(this.state.selected),
    );
  };

  render() {
    const { files } = this.props;

    const entries = Array.from(this.state.statuses.entries());
    const inProgress = entries.find(entry => entry[1].status === "UPLOADING");
    const uploaded = entries.filter(entry => entry[1].status === "UPLOADED")
      .length;
    const queued = entries.filter(entry => entry[1].status === "QUEUED").length;

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
        <div className={styles.previewArea}>
          <div className={styles.previewFrame}>
            {this.props.files.length > 0 && (
              <ImagePreview file={this.state.selected} />
            )}
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
            <div className={styles.action}>
              <Action label="Pick" keybinding="p" do={this.upload} />
            </div>
            <div className={styles.action}>
              <Action label="Dismiss" keybinding="d" do={this.upload} />
            </div>
            <div>
              {inProgress &&
                `Uploading ${inProgress[0].name}: ${inProgress[1].progress}%`}
              <br />
              {uploaded} Uploaded | {queued} Queued
            </div>
          </div>
        </div>
      </React.Fragment>
    );
  }
}

export default connect(state => ({
  api: state.api.host,
  headers: state.api.headers,
}))(Uploader);
