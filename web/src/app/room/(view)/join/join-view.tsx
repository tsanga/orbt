"use client";

import * as styles from "./join-view.css";
import useAuth from "@hooks/use-auth";
import IsolatedForm from "@/room/(shared)/isolated-form";
import InputButtonGroup from "@/(reusable)/input-button-group";
import { useState } from "react";
import useRoomContext from "@hooks/use-room-context";

export default function RoomJoinView() {
  const [state, dispatch] = useRoomContext();
  const { user } = useAuth();
  const [name, setName] = useState(user?.name);

  const join = () => {};

  return (
    <IsolatedForm>
      <h1 className={styles.title}>Join {state.room?.name}</h1>
      <InputButtonGroup
        id="set-user-name"
        placeholder="ronald mcdonald"
        buttonText="Join"
        onChange={(value) => setName(value)}
        onSubmit={join}
      />
    </IsolatedForm>
  );
}
