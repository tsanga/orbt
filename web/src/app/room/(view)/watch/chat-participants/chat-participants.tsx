import * as styles from "./chat-participants.css";

import { UserDisplayPartial, Status } from "@domain/models";
import RoomChatParticipant from "./chat-participant";
import RoomChatParticipantInviteButton from "./chat-participant-invite";
import { graphql, useFragment } from "react-relay";
import { chatParticipants$key } from "@gql/chatParticipants.graphql";

type Props = {
  room: chatParticipants$key;
};

export default function RoomChatParticipants({ room }: Props) {
  const data = useFragment(
    graphql`
      fragment chatParticipants on Room {
        id
        members {
          color {
            hex
          }
          user {
            name
            id
          }
        }
      }
    `,
    room
  );

  return (
    <aside className={styles.container}>
      {data.members.map((user) => (
        <RoomChatParticipant key={user.user.id} {...user}></RoomChatParticipant>
      ))}
      <RoomChatParticipantInviteButton room={data.id} />
    </aside>
  );
}

export const Skeleton = () => {
  return (
    <aside className={styles.container}>
      {[...Array(4)].map((e, i) => (
        <div className={styles.participantSkeleton} key={i}></div>
      ))}
    </aside>
  );
};
