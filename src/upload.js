// @flow

export default (bucket: string) => `
  <!doctype html>
    <html>
    <body>
      <h1>uploader test</h1>
      <h2>parameters</h2>
      <h3>Bucket: ${bucket}</h3>

      <hr/>

      <input type="file" id="file-input">
      <p id="status">Please select a file</p>
      <img id="preview" src="/images/default.png">

      <script src="static/client.js"></script>
    </body>
  </html>
`;
