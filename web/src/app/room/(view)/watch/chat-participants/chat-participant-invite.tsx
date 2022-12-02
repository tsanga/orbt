"use client";

import * as styles from "./chat-participant-invite.css";
import PlusIcon from "@assets/svg/icon/plus.svg";
import { graphql, useMutation } from "react-relay";
import { Room } from "@domain/models";
import useRoomContext from "@hooks/use-room-context";
import type { chatParticipantInviteMutation } from "@gql/chatParticipantInviteMutation.graphql";

type Props = {
  room: Room["id"];
};

export default function RoomChatParticipantInviteButton({ room }: Props) {
  const [_state, dispatch] = useRoomContext();

  const [commitMutation, inFlight] =
    useMutation<chatParticipantInviteMutation>(graphql`
      mutation chatParticipantInviteMutation($room: Id!) {
        createRoomInvite(room: $room) {
          token
        }
      }
    `);

  const onClick = () => {
    if (inFlight) return;

    commitMutation({
      variables: { room: room },
      onCompleted: (data) => {
        dispatch({
          type: "SET_GENERATED_INVITE_CODE",
          generatedInviteCode: data.createRoomInvite.token!!,
        });
      },
    });
  };

  return (
    <button className={styles.button} onClick={onClick}>
      <div className={styles.buttonIcon}>
        <PlusIcon />
      </div>
    </button>
  );
}
