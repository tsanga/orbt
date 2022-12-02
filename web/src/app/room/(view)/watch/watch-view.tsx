"use client";

import * as styles from "./watch.css";
import RoomTopBar from "./top-bar/room-top-bar";
import RoomChatBox from "./chat-box/chat-box";
import RoomChatParticipants, {
  Skeleton,
} from "./chat-participants/chat-participants";
import { Suspense } from "react";
import { graphql } from "relay-runtime";
import { PreloadedQuery, usePreloadedQuery } from "react-relay";
import type { watchViewRoomQuery as RoomQuery } from "@gql/watchViewRoomQuery.graphql";

type Props = {
  queryRef: PreloadedQuery<RoomQuery>;
};

export default function RoomWatchView({ queryRef }: Props) {
  const data = usePreloadedQuery<RoomQuery>(
    graphql`
      query watchViewRoomQuery($id: Id!) {
        room(room: $id) {
          id
          name

          ...chatParticipants
        }
      }
    `,
    queryRef
  );

  return (
    <main className={styles.main}>
      <section className={styles.leftSection}>
        <RoomTopBar />
      </section>
      <section className={styles.rightSection}>
        <RoomChatBox subheading={"Hello"} />
        <Suspense fallback={<Skeleton />}>
          <RoomChatParticipants room={data.room!!} />
        </Suspense>
      </section>
    </main>
  );
}
