// @flow

// For some reason webpack doesn't support putting wasm in the initial download
// so we have to create this shim module to actually access stuff.
import { Preview } from "editor-backend";
import { memory } from "editor-backend-wasm";

export { Preview, memory };
