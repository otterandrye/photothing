// @flow

import * as React from "react";

type Props = {|
  input: File,
  height: number,
  width: number,
  scale: number,
|};

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
        elmt.height = this.getVPixelHeight(); // this.props.height;
        elmt.width = this.getVPixelWidth(); // this.props.width;
        const ctx = elmt.getContext("2d");
        // $FlowFixMe
        ctx.webkitImageSmoothingEnabled = false;
        ctx.imageSmoothingEnabled = false;
        ctx.scale(this.props.scale, this.props.scale);

        const preview = Preview.new(
          this.getVPixelWidth(),
          this.getVPixelHeight(),
        );
        this.draw(ctx, memory, preview.pixels());

        const reader = new FileReader();
        reader.readAsArrayBuffer(this.props.input);
        reader.onload = event => {
          const data = new Uint8Array(event.target.result);
          preview.read(data, this.props.input.size);
          this.draw(ctx, memory, preview.pixels());
        };
      }
    });
  }

  getVPixelHeight = () => 4224; // Math.ceil(this.props.height * this.props.scale);
  getVPixelWidth = () => 6384; // Math.ceil(this.props.width * this.props.scale);
  getVPixelCount = () => this.getVPixelHeight() * this.getVPixelWidth();
  draw = (ctx: CanvasRenderingContext2D, memory, pixels) => {
    const imageData = new ImageData(
      new Uint8ClampedArray(memory.buffer, pixels, this.getVPixelCount() * 4),
      this.getVPixelWidth(),
      this.getVPixelHeight(),
    );
    ctx.putImageData(imageData, 0, 0);
  };

  render() {
    return (
      <canvas
        ref={this.canvas}
        style={{
          imageRendering: "pixelated",
          height: `${this.props.height}px`,
          width: `${this.props.width}px`,
        }}
      />
    );
  }
}
