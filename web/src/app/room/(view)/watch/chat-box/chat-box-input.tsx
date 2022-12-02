"use client";

import * as styles from "./chat-box-input.css";
import * as theme from "@theme/theme.css";
import PaperPlaneIcon from "@assets/svg/icon/paper-plane.svg";
import SmileyIcon from "@assets/svg/icon/smiley.svg";
import { graphql } from "relay-runtime";
import { useMutation } from "react-relay";
import { useState } from "react";
import type { chatBoxInputSendMessageMutation } from "@gql/chatBoxInputSendMessageMutation.graphql";

type Props = {
  room: string;
};

export default function ChatBoxInput({ room }: Props) {
  const [message, setMessage] = useState("");

  const [commitMutation, inFlight] =
    useMutation<chatBoxInputSendMessageMutation>(graphql`
      mutation chatBoxInputSendMessageMutation($room: Id!, $msg: String!) {
        sendChatMessage(room: $room, msg: $msg) {
          id
        }
      }
    `);

  const sendMessage = () => {
    if (inFlight) {
      return;
    }

    commitMutation({
      variables: { room: room, msg: message },
    });

    setMessage("");
  };

  return (
    <div className={styles.inputContainer}>
      <input
        className={styles.input}
        placeholder="Send a message..."
        value={message}
        onKeyDown={(e) => {
          if (e.key === "Enter") {
            sendMessage();
          }
        }}
        onChange={(e) => setMessage(e.target.value)}
      ></input>
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
            <button
              className={`${theme.button.link} ${theme.button.xs}`}
              onClick={sendMessage}
            >
              <PaperPlaneIcon />
            </button>
          </li>
        </ul>
      </div>
    </div>
  );
}
