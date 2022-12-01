"use client";

import * as styles from "./join-view.css";
import useAuth from "@hooks/use-auth";
import IsolatedForm from "@/room/(shared)/isolated-form";
import InputButtonGroup from "@/(reusable)/input-button-group";
import { useState } from "react";
import useRoomContext from "@hooks/use-room-context";
import { useMutation, graphql } from "react-relay";
import type { joinViewRoomMutation } from "@gql/joinViewRoomMutation.graphql";
import useAppContext from "@hooks/use-app-context";

export default function RoomJoinView() {
  const [roomState, roomDispatch] = useRoomContext();
  const [appState, appDispatch] = useAppContext();
  const { user } = useAuth();
  const [name, setName] = useState(user?.name);

  const [commitMutation, _] = useMutation<joinViewRoomMutation>(graphql`
    mutation joinViewRoomMutation($name: String) {
      joinRoom {
        id
      }

      setUserName(name: $name) {
        id
      }
    }
  `);

  const join = () => {
    commitMutation({
      variables: { name },
      onCompleted: (data) => {
        appDispatch(type: "SET_USER", user: { ...user, name: data.setUserName.name });
        roomDispatch({ type: "SET_IS_CREATING_ROOM", isCreatingRoom: false });
        roomDispatch({ type: "SET_IS_JOINING_ROOM", isJoiningRoom: false });
        roomDispatch({ type: "SET_ROOM", room: data.joinRoom });
      },
    });
  };

  return (
    <IsolatedForm>
      <h1 className={styles.title}>Join {state.room?.name}</h1>
      <InputButtonGroup
        id="set-user-name"
        placeholder="ronald mcdonald"
        buttonText="Join"
        onChange={(value) => setName(value.trim())}
        onSubmit={join}
      />
    </IsolatedForm>
  );
}
