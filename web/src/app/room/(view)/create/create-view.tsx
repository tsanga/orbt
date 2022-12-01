"use client";

import * as styles from "./create-view.css";
import * as theme from "@theme/theme.css";

import IsolatedForm from "../../(shared)/isolated-form";
import { useContext, useState } from "react";
import InputButtonGroup from "@/(reusable)/input-button-group";
import type { createViewRoomMutation } from "@gql/createViewRoomMutation.graphql";
import { useMutation, graphql } from "react-relay";
import useRoomContext from "@hooks/use-room-context";

export default function RoomCreateView() {
  const [_state, dispatch] = useRoomContext();
  const [roomName, setRoomName] = useState("");

  const [commitMutation, _] = useMutation<createViewRoomMutation>(
    graphql`
      mutation createViewRoomMutation($name: String) {
        createRoom(name: $name) {
          id
          name
        }
      }
    `
  );

  const create = () => {
    commitMutation({
      variables: { name: roomName },
      onCompleted: (data) => {
        dispatch({ type: "SET_IS_CREATING_ROOM", isCreatingRoom: false });
        dispatch({ type: "SET_IS_JOINING_ROOM", isJoiningRoom: true });
        dispatch({ type: "SET_ROOM", room: data.createRoom });
      },
      onError: (error) => {
        console.log(error);
      },
    });
  };

  return (
    <IsolatedForm>
      <h1 className={styles.title}>{roomName ? roomName : "Create Room"}</h1>
      <InputButtonGroup
        id="set-room-name"
        placeholder="ymcmb"
        buttonText="Create"
        onChange={(value) => setRoomName(value.trim())}
        onSubmit={create}
      />
    </IsolatedForm>
  );
}
