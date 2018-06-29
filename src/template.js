// @flow

type Page = {
  title: string,
  body: string,
};

export default ({ title, body }: Page) =>
  `<!doctype html><html lang="en"><head><meta charset="utf-8"><title>${title}</title><script src="static/client.js" async></script></head><body>${body}</body></html>`;
