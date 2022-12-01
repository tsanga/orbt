"use client";

import * as styles from "./page.css";

import { Suspense, lazy } from "react";
import { useContext } from "react";
import { RoomContext } from "./context";
import RoomWatchView from "./(view)/watch/watch-view";
import RoomCreateView from "./(view)/create/create-view";

export default function RoomPage() {
  const roomContext = useContext(RoomContext);
  const creatingRoom = <RoomCreateView />;

  if (roomContext?.isCreatingRoom) {
    return creatingRoom;
  } else if (roomContext?.room) {
    return (
      <Suspense fallback={<div>Loading...</div>}>
        <RoomWatchView />
      </Suspense>
    );
  } else if (roomContext?.isJoiningRoom) {
    return <h1>Joining room...</h1>;
  } else {
    return creatingRoom;
  }
}
