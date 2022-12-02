"use client";

import * as styles from "./page.css";

import { Suspense, lazy, useCallback } from "react";
import { useContext } from "react";
import { RoomContext } from "./context";
import RoomWatchView from "./(view)/watch/watch-view";
import RoomCreateView from "./(view)/create/create-view";
import RoomJoinView from "./(view)/join/join-view";
import useRoomContext from "@hooks/use-room-context";
import { RelayContext } from "@/util/relay-environment-provider-wrapper";
import { loadQuery, PreloadedQuery, useQueryLoader } from "react-relay";
import type {
  watchViewRoomQuery as RoomQueryType,
  watchViewRoomQuery,
} from "@gql/watchViewRoomQuery.graphql";
import { useMemo } from "react";
import { Environment } from "relay-runtime";
import { useEffect } from "react";
import { useState } from "react";
import useAuth from "@hooks/use-auth";
import { useRef } from "react";

const RoomQuery = require("@gql/watchViewRoomQuery.graphql");

export default function RoomPage() {
  const [state, _dispatch] = useRoomContext();
  const [queryRef, loadQuery] = useQueryLoader<RoomQueryType>(RoomQuery);
  const relayState = useContext(RelayContext)!!;
  const creatingRoom = <RoomCreateView />;
  const env = relayState.environment;
  const id = state.room?.id;

  useEffect(() => {
    if (id) {
      loadQuery({ id });
    }
  }, [env, id]);

  if (state?.isCreatingRoom) {
    return creatingRoom;
  } else if (state?.isJoiningRoom && state?.room) {
    return <RoomJoinView />;
  } else if (state?.room?.id && queryRef) {
    return <RoomWatchView queryRef={queryRef} />;
  } else {
    return creatingRoom;
  }
}
