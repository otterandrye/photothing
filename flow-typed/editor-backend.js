declare module "editor-backend" {
  declare type PreviewType = {
    new: (number, number) => PreviewType,
    pixels: () => number,
    read: (Uint8Array, number) => void,
  }

  declare export var Preview: PreviewType;
}

declare module "editor-backend-wasm" {
  declare export var memory: { buffer: ArrayBuffer };
}