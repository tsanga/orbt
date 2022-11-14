"use client";

import * as styles from "./chat-tool-bar.css";
import { theme } from "@theme";
import LogoutIcon from "@assets/svg/icon/logout.svg";
import CogIcon from "@assets/svg/icon/cog.svg";
import LineArrowAway from "@assets/svg/icon/line-arrow-away.svg";

export default function ChatToolBar() {
  return (
    <ul className={styles.toolbar}>
      <li className={styles.minimizeChatButton}>
        <button
          className={`${theme.button.link} ${theme.button.sm} ${theme.textColor.dim}`}
        >
          <LineArrowAway />
        </button>
      </li>
      <li className={styles.cog}>
        <button
          className={`${theme.button.link} ${theme.button.sm} ${theme.textColor.dim}`}
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
