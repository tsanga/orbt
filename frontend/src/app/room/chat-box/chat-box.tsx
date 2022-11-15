import React from "react";
import * as styles from "./chat-box.css";
import ChatToolBar from "./chat-tool-bar";
import ChatBoxInput from "./chat-box-input";
import ChatIcon from "@assets/svg/icon/chat.svg";

type Props = {
  subheading?: React.ReactNode;
};

const ChatBox = ({ subheading }: Props) => {
  return (
    <div className={styles.chatContainer}>
      <header className={styles.chatHeader}>
        <ChatToolBar />
        <h4 className={styles.chatHeaderHeading}>
          <ChatIcon />
          &nbsp;Chat
        </h4>
        {!!subheading && (
          <h5 className={styles.chatHeaderSubheading}>{subheading}</h5>
        )}
      </header>
      <main className={styles.main}></main>
      <footer className={styles.footer}>
        <ChatBoxInput />
      </footer>
    </div>
  );
};

export default ChatBox;
