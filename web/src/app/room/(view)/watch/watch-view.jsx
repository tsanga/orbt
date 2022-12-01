import * as styles from "./watch.css";
import RoomTopBar from "./top-bar/room-top-bar";
import RoomChatBox from "./chat-box/chat-box";
import RoomChatParticipants, {
  Skeleton,
} from "./chat-participants/chat-participants";
import { Suspense } from "react";

export default function RoomWatchView() {
  return (
    <main className={styles.main}>
      <section className={styles.leftSection}>
        <RoomTopBar />
      </section>
      <section className={styles.rightSection}>
        <RoomChatBox subheading={"Hello"} />
        <Suspense fallback={<Skeleton />}>
          {/* @ts-expect-error Server Component */}
          <RoomChatParticipants />
        </Suspense>
      </section>
    </main>
  );
}
