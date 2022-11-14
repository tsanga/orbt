import React from "react";
import * as styles from "./chat-box.css";
import ChatToolBar from "./chat-tool-bar";

type Props = {
  subheading?: React.ReactNode;
};

const ChatBox = ({ subheading }: Props) => {
  return (
    <div className={styles.chatContainer}>
      <header className={styles.chatHeader}>
        <ChatToolBar />
        <h4 className={styles.chatHeaderHeading}>Chat</h4>
        {!!subheading && (
          <h5 className={styles.chatHeaderSubheading}>{subheading}</h5>
        )}
      </header>
    </div>
  );
};

export default ChatBox;
