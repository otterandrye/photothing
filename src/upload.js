// @flow

type Aws = {
  bucket: string,
  id: string,
  secret: string,
};

export default ({ bucket, id, secret }: Aws) => `
  <!doctype html>
    <html>
    <body>
      <h1>uploader test</h1>
      <h2>parameters</h2>
      <h3>Bucket: ${bucket}</h3>
      <h3>ID: ${id}</h3>
      <h3>SECRET: ${secret}</h3>

      <hr/>

      <input type="file" id="file-input">
      <p id="status">Please select a file</p>
      <img id="preview" src="/images/default.png">

      <script src="static/client.js"></script>
    </body>
  </html>
`;
// bucket: string, id: string, secret: string
