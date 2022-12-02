import { Modal } from "@/room/(shared)/modal";
import { inviteModal$key } from "@gql/inviteModal.graphql";
import useRoomContext from "@hooks/use-room-context";
import { graphql, useFragment } from "react-relay";
import * as styles from "./invite-modal.css";

type Props = {
  inviteCode: string;
  room: inviteModal$key;
};

export default function RoomInviteModal({ inviteCode, room }: Props) {
  const [_state, dispatch] = useRoomContext();

  const data = useFragment(
    graphql`
      fragment inviteModal on Room {
        name
      }
    `,
    room
  );

  const copy = (text: string) => {
    navigator.clipboard.writeText(text);
  };

  return (
    <Modal
      onClose={() => {
        dispatch({
          type: "SET_GENERATED_INVITE_CODE",
          generatedInviteCode: "",
        });
      }}
    >
      <h1 className={`${styles.title}`}>Invite peeps to {data.name}</h1>
    </Modal>
  );
}
