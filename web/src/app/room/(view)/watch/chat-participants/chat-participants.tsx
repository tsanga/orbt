import * as styles from "./chat-participants.css";

import { UserDisplayPartial, Status } from "@domain/models";
import RoomChatParticipant from "./chat-participant";

const getConnectedUsers = async (): Promise<UserDisplayPartial[]> => {
  // add fake delay
  return await new Promise((r) =>
    setTimeout(
      () =>
        r([
          {
            id: "0",
            name: "bizarre",
            profilePicture:
              "https://cdn.discordapp.com/avatars/186986309376671744/ffa506174c7caf71da388395848e251f.png",
            status: Status.CONNECTED,
          },
          {
            id: "1",
            name: "onah_",
            profilePicture:
              "https://cdn.discordapp.com/avatars/120711274291003393/a3f8512ee6ca7d7c7948bca81db7c4e2.png",
            status: Status.AWAY,
          },
          {
            id: "2",
            name: "mandingo",
            profilePicture:
              "https://cdn.discordapp.com/avatars/242398375725629441/7a142ea30e5b2953ac9282e151087955.png",
            status: Status.CONNECTING,
          },
          {
            id: "3",
            name: "perc",
            profilePicture:
              "https://cdn.discordapp.com/avatars/414637230800502806/346ed99c0106a400f2fde08a7d00cc07.png",
            status: Status.CONNECTED,
          },
          {
            id: "",
            name: "chicken little",
            profilePicture:
              "https://cdn.discordapp.com/avatars/494295586950873088/3f652b4277d0e81c23eeb76539993c06.png",
            status: Status.DISCONNECTED,
          },
        ]),
      500
    )
  );
};

export default async function RoomChatParticipants() {
  const users = await getConnectedUsers();

  const sortedUsers = users.sort(
    (a, b) =>
      Object.keys(Status).indexOf(a.status.toUpperCase()) -
      Object.keys(Status).indexOf(b.status.toUpperCase())
  );

  return (
    <aside className={styles.container}>
      {sortedUsers.map((user) => (
        /* todo: change this for uid or some shit */
        <RoomChatParticipant {...user} key={user.name} />
      ))}
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
