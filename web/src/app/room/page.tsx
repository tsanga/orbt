"use client";

import * as styles from "./page.css";

import { Suspense, lazy } from "react";
import { useContext } from "react";
import { RoomContext } from "./context";
import RoomWatchView from "./(view)/watch/watch-view";
import RoomCreateView from "./(view)/create/create-view";
import RoomJoinView from "./(view)/join/join-view";
import useRoomContext from "@hooks/use-room-context";
import { RelayContext } from "@/util/relay-environment-provider-wrapper";
import { loadQuery } from "react-relay";
import type { watchViewRoomQuery as RoomQueryType } from "@gql/watchViewRoomQuery.graphql";
const RoomQuery = require("@gql/watchViewRoomQuery.graphql");

export default function RoomPage() {
  const [state, _dispatch] = useRoomContext();
  const relayState = useContext(RelayContext)!!;
  const creatingRoom = <RoomCreateView />;

  if (state?.isCreatingRoom) {
    return creatingRoom;
  } else if (state?.isJoiningRoom && state?.room) {
    return <RoomJoinView />;
  } else if (state?.room && state?.room?.id) {
    const initialQueryRef = loadQuery<RoomQueryType>(
      relayState.environment,
      RoomQuery,
      {
        id: state.room.id,
      }
    );

    return <RoomWatchView queryRef={initialQueryRef} />;
  } else {
    return creatingRoom;
  }
}
