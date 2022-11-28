"client only";

import { UserDisplayPartial } from "@domain/models";
import * as styles from "./chat-participant.css";

type Props = UserDisplayPartial;

export default function RoomChatParticipant({
  name,
  profilePicture,
  status,
}: Props) {
  return (
    <div
      className={styles.participant[status]}
      style={{ backgroundImage: `url(${profilePicture})` }}
    />
  );
}
