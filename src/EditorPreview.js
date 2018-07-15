// @flow

import * as React from "react";

type Props = {
  height: number,
  width: number,
  scale: number,
};

export default class EditorPreview extends React.Component<Props> {
  canvas: { current: null | HTMLCanvasElement } = React.createRef();

  static defaultProps = {
    height: 400,
    width: 400,
    scale: typeof window !== "undefined" ? window.devicePixelRatio || 1 : 1,
  };

  componentDidMount() {
    import("./editor").then(({ Preview, memory }) => {
      const elmt = this.canvas.current;
      if (elmt) {
        elmt.height = this.props.height;
        elmt.width = this.props.width;
        const ctx = elmt.getContext("2d");
        ctx.scale(this.props.scale, this.props.scale);

        const preview = Preview.new(
          this.getVPixelHeight(),
          this.getVPixelWidth(),
        );
        const imageData = new ImageData(
          new Uint8ClampedArray(
            memory.buffer,
            preview.pixels(),
            this.getVPixelCount() * 4,
          ),
          this.getVPixelHeight(),
          this.getVPixelWidth(),
        );
        ctx.putImageData(imageData, 0, 0);
      }
    });
  }

  getVPixelHeight = () => Math.ceil(this.props.height * this.props.scale);

  getVPixelWidth = () => Math.ceil(this.props.width * this.props.scale);

  getVPixelCount = () => this.getVPixelHeight() * this.getVPixelWidth();

  render() {
    return (
      <canvas
        ref={this.canvas}
        style={{
          height: `${this.props.height}px`,
          width: `${this.props.width}px`,
        }}
      />
    );
  }
}
