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
import useRoomContext from "@hooks/use-room-context";
import RoomInviteModal from "./invite-modal/invite-modal";

type Props = {
  queryRef: PreloadedQuery<RoomQuery>;
};

const useRoomSubscription = (id: string) => {
  const config = useMemo(() => {
    return {
      subscription: graphql`
        subscription watchViewRoomSubscription($id: Id!) {
          room(room: $id) {
            id
            ...chatParticipants
            ...chatBoxMessages
            ...roomTopBarTitle
            ...inviteModal
          }
        }
      `,
      variables: { id },
    };
  }, [id]);

  return useSubscription(config);
};

export default function RoomWatchView({ queryRef }: Props) {
  const [state, dispatch] = useRoomContext();

  useRoomSubscription(queryRef.variables.id);

  const data = usePreloadedQuery<RoomQuery>(
    graphql`
      query watchViewRoomQuery($id: Id!) {
        room(room: $id) {
          id
          name

          ...chatParticipants
          ...chatBoxMessages
          ...roomTopBarTitle
          ...inviteModal
        }
      }
    `,
    queryRef
  );

  return (
    <main className={styles.main}>
      {state?.generatedInviteCode && (
        <RoomInviteModal
          inviteCode={state.generatedInviteCode}
          room={data.room!!}
        />
      )}

      <section className={styles.leftSection}>
        <RoomTopBar room={data.room!!} />
      </section>
      <section className={styles.rightSection}>
        <RoomChatBox subheading={"Hello"} room={data.room!!} />
        <Suspense fallback={<Skeleton />}>
          <RoomChatParticipants room={data.room!!} />
        </Suspense>
      </section>
    </main>
  );
}
