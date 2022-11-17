"client only";

import * as styles from "./chat-box-input.css";
import { theme } from "@theme";
import PaperPlaneIcon from "@assets/svg/icon/paper-plane.svg";
import SmileyIcon from "@assets/svg/icon/smiley.svg";

export default function ChatBoxInput() {
  return (
    <div className={styles.inputContainer}>
      <input className={styles.input} placeholder="Send a message..."></input>
      <div className={styles.toolbarContainer}>
        <ul className={styles.toolbar}>
          <li>
            <button
              className={`${theme.button.link} ${theme.textColor.dim} ${theme.button.xs}`}
            >
              <SmileyIcon />
            </button>
          </li>
          <li>
            <button className={`${theme.button.link} ${theme.button.xs}`}>
              <PaperPlaneIcon />
            </button>
          </li>
        </ul>
      </div>
    </div>
  );
}
