// @flow

/* eslint-disable import/prefer-default-export */

type Page<T> = {|
  key: ?string,
  next_key: ?string,
  items: Array<T>
|};

export type { Page };
