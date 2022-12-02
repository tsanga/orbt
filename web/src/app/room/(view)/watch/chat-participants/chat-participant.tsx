"client only";

import { RoomParticipant } from "@domain/models";
import * as styles from "./chat-participant.css";

type Props = RoomParticipant;

export default function RoomChatParticipant({ user }: Props) {
  return (
    <div
      className={styles.participant.connected}
      style={{
        backgroundImage: `url(https://picsum.photos/seed/${user.id}/200)`,
      }}
    />
  );
}
