"use client";

import React from "react";
import * as styles from "./chat-box.css";
import ChatToolBar from "./chat-tool-bar";
import ChatBoxInput from "./chat-box-input";
import ChatIcon from "@assets/svg/icon/chat.svg";
import Planets from "@assets/svg/planets.svg";
import { graphql, useFragment, usePaginationFragment } from "react-relay";
import { chatBoxMessages$key } from "@gql/chatBoxMessages.graphql";

type Props = {
  subheading?: React.ReactNode;
  room: chatBoxMessages$key;
};

const ChatBox = ({ subheading, room }: Props) => {
  const data = usePaginationFragment(
    graphql`
      fragment chatBoxMessages on Room
      @refetchable(queryName: "chatBoxMessagesPaginationQuery") {
        id
        messages(first: $count, after: $cursor)
          @connection(key: "chatBoxMessages_messages") {
          edges {
            node {
              id
              time
              msg
              author
            }
          }
        }
        members {
          user {
            id
            name
          }
        }
      }
    `,
    room
  );

  const usersMapped = data.members
    .map((member) => member.user)
    .reduce((acc, user) => {
      acc[user.id] = user;
      return acc;
    }, {} as Record<string, { id: string; name: string }>);

  return (
    <div className={styles.chatContainer} data-testid="chat-box">
      <Planets className={styles.planets} />
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
      <main className={styles.main}>
        {data.messages.map((message) => (
          <div key={message.id}>
            {usersMapped[message.author]?.name}: {message.msg}
          </div>
        ))}
      </main>
      <footer className={styles.footer}>
        <ChatBoxInput room={data.id} />
      </footer>
    </div>
  );
};

export default ChatBox;
