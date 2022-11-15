import * as styles from "./chat-participants.css";

import { UserDisplayPartial, Status } from "@types";

const getConnectedUsers = async (): Promise<UserDisplayPartial[]> => {
  // add fake delay
  await new Promise((r) => setTimeout(r, 5000));

  return [{ name: "alex", profilePicture: "", status: Status.Connected }];
};

export default async function RoomChatParticipants() {
  const users = await getConnectedUsers();

  return <aside className={styles.container}>{}</aside>;
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
