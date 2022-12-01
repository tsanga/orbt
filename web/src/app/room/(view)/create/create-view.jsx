"use client";

import * as styles from "./create-view.css";
import * as theme from "@theme/theme.css";

import IsolatedForm from "../../(shared)/isolated-form";
import { useState } from "react";
import InputButtonGroup from "@/(reusable)/input-button-group";

export default function RoomCreateView() {
  const [roomName, setRoomName] = useState("");

  return (
    <IsolatedForm>
      <h1 className={styles.title}>{roomName ? roomName : "Create Room"}</h1>
      <InputButtonGroup
        id="set-room-name"
        placeholder="ymcmb"
        onChange={(value) => setRoomName(value)}
      />
    </IsolatedForm>
  );
}
