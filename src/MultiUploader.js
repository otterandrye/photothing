// @flow

import * as React from "react";
import { connect } from "react-redux";

import css from "./MultiUploader.css";
import ImagePreview from "./ImagePreview";
import ImageFrame from "./ImageFrame";
import FileSize from "./FileSize";
import guid from "./guid";

type Props = {|
  api: string,
  headers: { [string]: string },
  edit: File => void,
|};

type Upload = {|
  file: File,
  status: "SELECTED" | "IN_PROGRESS" | "DONE",
  uploadStatus: ?string,
|};

type GUID = string;
type State = {|
  uploads: { [GUID]: Upload },
|};

const formatter = new Intl.NumberFormat(undefined, { style: "percent" });

class MultiUploader extends React.Component<Props, State> {
  state: State = { uploads: {} };

  getSignedRequest = (api: string, headers: { [string]: string }, file: File) =>
    fetch(`${api}/api/upload`, {
      method: "POST",
      headers,
      // $FlowFixMe: TODO: How do we post JSON via fetch?
      body: {
        filename: file.name,
        file_type: file.type,
      },
    })
      .then(res => res.json())
      .then(pendingUpload => pendingUpload.upload.url);

  drop = (evt: SyntheticDragEvent<HTMLDivElement>) => {
    evt.stopPropagation();
    evt.preventDefault();

    this.addUpload(evt.dataTransfer.files);
  };

  ignore = (evt: SyntheticEvent<*>) => {
    evt.stopPropagation();
    evt.preventDefault();
  };

  handler = (evt: SyntheticEvent<HTMLInputElement>) => {
    this.addUpload(evt.currentTarget.files);
  };

  addUpload = (fileList: FileList) => {
    const newFiles = {};
    Array.from(fileList).forEach(file => {
      newFiles[guid()] = { status: "SELECTED", file };
    });

    this.setState(state => ({
      uploads: {
        ...state.uploads,
        ...newFiles,
      },
    }));
  };

  delete = (id: GUID) => {
    this.setState(state => {
      const temp = Object.assign({}, state.uploads);
      delete temp[id];
      return {
        uploads: temp,
      };
    });
  };

  upload = (id: GUID) => {
    const upload = this.state.uploads[id];

    const mark = (status, uploadStatus) =>
      this.setState(state => ({
        uploads: {
          ...state.uploads,
          [id]: {
            file: state.uploads[id].file,
            status,
            uploadStatus,
          },
        },
      }));

    mark("IN_PROGRESS", 0);

    this.getSignedRequest(this.props.api, this.props.headers, upload.file).then(
      signedRequest => {
        const xhr = new XMLHttpRequest();
        xhr.upload.addEventListener(
          "progress",
          (evt: ProgressEvent) => {
            mark("IN_PROGRESS", formatter.format(evt.loaded / evt.total));
          },
          false,
        );
        xhr.open("PUT", signedRequest);
        xhr.onreadystatechange = () => {
          if (xhr.readyState === 4) {
            if (xhr.status === 200) {
              mark("DONE", 100);
            }
          }
        };
        xhr.send(upload.file);
      },
      e => {
        console.error(`Upload failed: '${e}'`);
        mark("SELECTED", 0);
      },
    );
  };

  render() {
    const { uploads } = this.state;
    return (
      <React.Fragment>
        <div
          onDrop={this.drop}
          onDragEnter={this.ignore}
          onDragOver={this.ignore}
          className={css.dropBox}
        >
          <h3>Drop files here!</h3>
          <label htmlFor="multiuploader" className={css.button}>
            Or Select Files
            <input
              id="multiuploader"
              className={css.hidden}
              type="file"
              multiple
              accept="image/jpeg, image/x-adobe-dng"
              onChange={this.handler}
            />
          </label>
        </div>

        <div className={css.uploads}>
          {(Object.entries(uploads): any).map(([id, info]: [GUID, Upload]) => (
            <div key={id} className={css.upload}>
              <ImageFrame>
                <ImagePreview file={info.file} />
              </ImageFrame>
              <div className={css.meta}>
                <div className={css.title}>{info.file.name}</div>
                {info.status === "SELECTED" && (
                  <div
                    className={css.uploadButton}
                    onClick={() => this.upload(id)}
                  >
                    Upload
                  </div>
                )}
                {info.status === "IN_PROGRESS" && (
                  <div className={css.uploadStatus}>{info.uploadStatus}</div>
                )}
                {info.status === "DONE" && (
                  <div className={css.uploadStatus}>Uploaded!</div>
                )}
                <small className={css.size}>
                  <FileSize size={info.file.size} />
                </small>
              </div>
              <div
                className={css.edit}
                onClick={() => this.props.edit(info.file)}
              >
                {"\u270E"}
              </div>
              <div className={css.x} onClick={() => this.delete(id)}>
                {"\xD7"}
              </div>
            </div>
          ))}
        </div>
      </React.Fragment>
    );
  }
}

export default connect(({ api }) => ({ api: api.host, headers: api.headers }))(
  MultiUploader,
);
