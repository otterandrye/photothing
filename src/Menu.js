// @flow

import * as React from "react";
import { connect } from "react-redux";
import { logout as logoutAction, navigate as navigationAction } from "./State";
import { type Route } from "./routes";
import styles from "./Menu.css";
import Action from "./Action";
import FilePicker from "./FilePicker";

type Props = {|
  +page: string,
  +email: string,
  +logout: () => void,
  +navigate: Route => void,
|};

const Menu = ({ page, email, logout, navigate }: Props) => {
  const libraryAction = () => {
    if (page !== "LIBRARY") {
      navigate({ page: "LIBRARY" });
    }
  };
  const albumsAction = () => {
    if (page !== "ALBUMS") {
      navigate({ page: "ALBUMS" });
    }
  };
  return (
    <div className={styles.menuSizing}>
      <div className={`${styles.menu} ${styles.menuSizing}`}>
        <div>
          <div className={styles.logoBlock}>
            <div className={styles.logo}>{"\u23e3"}</div>
            <div className={styles.brand}>Chroma</div>
          </div>
          <ul className={styles.nav}>
            <li
              className={page === "LIBRARY" ? styles.active : undefined}
              onClick={libraryAction}
            >
              <Action label="Library" keybinding="l" do={libraryAction} />
              <span className={styles.description}>
                Your complete collection
              </span>
            </li>
            <li
              className={page === "ALBUMS" ? styles.active : undefined}
              onClick={albumsAction}
            >
              <Action label="Albums" keybinding="a" do={albumsAction} />
              <span className={styles.description}>Show off your work</span>
            </li>
            <FilePicker
              accept={["image/jpeg", "image/x-adobe-dng"]}
              onSelect={() => {}}
            >
              {open => (
                <li onClick={open}>
                  <Action label="Upload" keybinding="u" do={open} />
                  <span className={styles.description}>
                    Or just drag-n-drop anywhere
                  </span>
                </li>
              )}
            </FilePicker>
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
};

export default connect(
  state => ({
    page: state.route.page,
    email: state.auth.email,
  }),
  { logout: logoutAction, navigate: navigationAction },
)(Menu);
