"use client";

import * as styles from "./chat-tool-bar.css";
import { theme } from "@theme";
import LogoutIcon from "@assets/svg/icon/logout.svg";
import CogIcon from "@assets/svg/icon/cog.svg";
import ArrowRightIcon from "@assets/svg/icon/arrow-right.svg";
import LineIcon from "@assets/svg/icon/line.svg";

export default function ChatToolBar() {
  return (
    <ul className={styles.toolbar}>
      <li className={styles.minimizeChatButton}>
        <button
          className={`${theme.button.link} ${theme.button.sm} ${theme.textColor.accentPrimary}`}
        >
          <LineIcon />
          <ArrowRightIcon className={styles.minimizeChatButtonArrow} />
        </button>
      </li>
      <li className={styles.cog}>
        <button
          className={`${theme.button.link} ${theme.button.sm} ${theme.textColor.accentPrimary}`}
        >
          <CogIcon />
        </button>
      </li>
      <li>
        <button className={`${theme.button.primary} ${theme.button.sm}`}>
          <LogoutIcon />
          &nbsp; Leave Room
        </button>
      </li>
    </ul>
  );
}
