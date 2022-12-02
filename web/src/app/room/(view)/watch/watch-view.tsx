"use client";

import * as styles from "./watch.css";
import RoomTopBar from "./top-bar/room-top-bar";
import RoomChatBox from "./chat-box/chat-box";
import RoomChatParticipants, {
  Skeleton,
} from "./chat-participants/chat-participants";
import { Suspense, useMemo } from "react";
import { graphql } from "relay-runtime";
import {
  PreloadedQuery,
  usePreloadedQuery,
  useSubscription,
} from "react-relay";
import type { watchViewRoomQuery as RoomQuery } from "@gql/watchViewRoomQuery.graphql";

type Props = {
  queryRef: PreloadedQuery<RoomQuery>;
};

const useRoomSubscription = (id: string) => {
  const config = useMemo(() => {
    return {
      subscription: graphql`
        subscription watchViewRoomSubscription($id: Id!) {
          room(id: $id) {
            id
            ...chatParticipants
            ...chatBoxMessages
          }
        }
      `,
      variables: { id },
    };
  }, [id]);

  return useSubscription(config);
};

export default function RoomWatchView({ queryRef }: Props) {
  useRoomSubscription(queryRef.variables.id);

  const data = usePreloadedQuery<RoomQuery>(
    graphql`
      query watchViewRoomQuery($id: Id!) {
        room(room: $id) {
          id
          name

          ...chatParticipants
          ...chatBoxMessages
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
