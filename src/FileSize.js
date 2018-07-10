// @flow

const round = (num, dec) => {
  const tens = 10 ** dec;
  return Math.round(num * tens) / tens;
};

const formatter = new Intl.NumberFormat();

export default ({ size }: { size: number }) => {
  const kb = round(size / 1024 ** 1, 1);
  const mb = round(size / 1024 ** 2, 1);
  const gb = round(size / 1024 ** 3, 1);
  if (kb <= 1) {
    return `${formatter.format(size)} bytes`;
  }
  if (mb <= 1) {
    return `${formatter.format(kb)} KiB`;
  }
  if (gb <= 1) {
    return `${formatter.format(mb)} MiB`;
  }
  return `${formatter.format(gb)} GiB`;
};
