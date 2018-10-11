// @flow

import * as React from "react";
import { connect } from "react-redux";
import { logout as logoutAction } from "./State";
import styles from "./Menu.css";
import Action from "./Action";

const libraryAction = () => {
  // eslint-disable-next-line no-alert
  alert("Library!");
};
const albumsAction = () => {
  // eslint-disable-next-line no-alert
  alert("Albums!");
};
const uploadAction = () => {
  // eslint-disable-next-line no-alert
  alert("Upload!");
};

type Props = {|
  +email: string,
  +logout: () => void,
|};

const Menu = ({ email, logout }: Props) => (
  <div className={styles.menuSizing}>
    <div className={`${styles.menu} ${styles.menuSizing}`}>
      <div>
        <div className={styles.logoBlock}>
          <div className={styles.logo}>{"\u23e3"}</div>
          <div className={styles.brand}>Photothing</div>
        </div>
        <ul className={styles.nav}>
          <li className={styles.active}>
            <Action label="Library" keybinding="l" do={libraryAction} />
            <span className={styles.description}>Your complete collection</span>
          </li>
          <li onClick={albumsAction}>
            <Action label="Albums" keybinding="a" do={albumsAction} />
            <span className={styles.description}>Show off your work</span>
          </li>
          <li onClick={uploadAction}>
            <Action label="Upload" keybinding="u" do={uploadAction} />
            <span className={styles.description}>
              Or just drag-n-drop anywhere
            </span>
          </li>
        </ul>
      </div>
      <div>
        <div className={styles.user}>{email}</div>
        <div onClick={logout} className={styles.logout}>
          Not you?
        </div>
      </div>
    </div>
  </div>
);

export default connect(
  state => ({
    email: state.auth.email,
  }),
  { logout: logoutAction },
)(Menu);
