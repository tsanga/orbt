"client only";

import * as styles from "./chat-participant-invite.css";
import PlusIcon from "@assets/svg/icon/plus.svg";

export default function RoomChatParticipantInviteButton() {
  return (
    <button className={styles.button}>
      <div className={styles.buttonIcon}>
        <PlusIcon />
      </div>
    </button>
  );
}
